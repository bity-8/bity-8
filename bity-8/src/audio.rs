// TODO: take care of all the possible errors (unwraps).
extern crate sdl2;
extern crate rand;
use audio::rand::prelude::*;

use sdl2::audio::AudioSpecDesired;
use sdl2::audio::AudioDevice;
use sdl2::audio::AudioCallback;
use self::sdl2::Sdl;
use memory as mem;
use cartridge as cart;

const SAMPLES:   u32 = 256;
const BITY_SAMP: f32 = 128f32;
const SPS:       u32 = 240;   // samples per second, 60 per frame
const SOFTENER:  f32 = 4000f32;
const PIANO_LEN: usize = 88;
const MAX_VOLUME: usize = 16;

// Piano frequencies taken from here: A0 - C8
// http://www.sengpielaudio.com/calculator-notenames.htm
// Then rounded (with round)
const PIANO_FREQS_INT: [u32; PIANO_LEN] =
[
    28,   29,   31,   33,   35,   37,   39,   41,   44,   46,   49,   52,
    55,   58,   62,   65,   69,   73,   78,   82,   87,   92,   98,  104,
    110,  117,  123,  131,  139,  147,  156,  165,  175,  185,  196,  208,
    220,  233,  247,  262,  277,  294,  311,  330,  349,  370,  392,  415,
    440,  466,  494,  523,  554,  587,  622,  659,  698,  740,  784,  831,
    880,  932,  988,  1046, 1109, 1175, 1245, 1319, 1397, 1480, 1568, 1661,
    1760, 1865, 1976, 2093, 2217, 2349, 2489, 2637, 2794, 2960, 3136, 3322,
    3520, 3729, 3951, 4186
];

const INSTRUMENTS_LEN: usize = 8;
const INSTRUMENTS: [mem::MemLoc; INSTRUMENTS_LEN] = 
[
    mem::OFF_INS1, mem::OFF_INS2, mem::OFF_INS3, mem::OFF_INS4,
    mem::OFF_INS5, mem::OFF_INS6, mem::OFF_INS7, mem::OFF_INS8,
];

pub struct Wave {
    current_index: usize,
    channel: usize
}

impl AudioCallback for Wave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let notes = mem::get_sub_area(mem::LOC_HARD, mem::OFF_NOTES);

        let note       = ( notes[self.channel*4+1]               ) as usize;
        let instrument = ((notes[self.channel*4] >> 4) & 0b0111u8) as usize;
        let volume     = ( notes[self.channel*4]       & 0b1111u8) as usize;

        //println!("c: {}, i: {}, vol: {}, not: {}", self.channel, instrument, volume, note);

        let volume = (volume % MAX_VOLUME) as i16;
        let wave_data = mem::get_sub_area(mem::LOC_INST, INSTRUMENTS[instrument % INSTRUMENTS_LEN].clone());
        let period = ((SPS * SAMPLES) / PIANO_FREQS_INT[note % PIANO_LEN]) as f32;

        // next: cache the last index.
        for x in 0..SAMPLES {
            let ind = (x as f32 % period / period * BITY_SAMP as f32) as usize;

            // some funky conversion between unsigned to signed.
            let amp = wave_data[(ind + self.current_index) % 128] as i16;
            let amp = if amp == 0 {
                let r: i8 = random();
                if r == -128 { -127i16 } else { r as i16 }
            } else { amp - 128 };

            let amp = amp * volume;
            out[x as usize] = amp as f32 / SOFTENER;
        }

        self.current_index += (SAMPLES as f32 % period / period * BITY_SAMP as f32) as usize;
        self.current_index %= 128usize;
    }
}

pub struct Channel {
    pub device: AudioDevice<Wave>,
}

impl Channel {
    pub fn new(sdl_context: &mut Sdl, channel_num: usize) -> Channel {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some((SPS * SAMPLES) as i32), // 128 * 60 = 7_680
            channels: Some(1u8),
            samples: Some(SAMPLES as u16)
        };

        audio_subsystem.open_queue::<i16, _>(None, &desired_spec).unwrap();
        let device = audio_subsystem.open_playback(None, &desired_spec, |_spec| {
            // Show obtained AudioSpec
            // println!("{:?}", spec);

            // initialize the audio callback
            Wave {
                current_index: 0,
                channel: channel_num % 4
            }
        }).unwrap();
        // println!("{:?}", device.spec());

        Channel {
            device: device,
        }
    }
}

// Updates the current note playing in memory.
pub fn update_mem_note() {
    let notes = mem::get_sub_area(mem::LOC_HARD, mem::OFF_NOTES);

    // 2 notes per channel. 2 bytes per note. 4 channels. So... 16 bytes.
    for i in 0..4 {
        notes[i*4+0] = notes[i*4+2]; // move next to current note.
        notes[i*4+1] = notes[i*4+3]; // move next to current note.
    }
}

// Updates the measure/sfx as it is playing in memory.
// DOES NOT GO TO THE NEXT SFX. That is something abstracted with music.
// This also doesn't load a measure. See "play_measure".
pub fn update_mem_measure() {
    // TODO: make this function cleaner.
    // TODO: be positive that the looping logic works.

    // reserved . note index  . time left X 4 channels
    // 00         00_0000     . 0000_0000
    let ctrl = mem::get_sub_area(mem::LOC_HARD, mem::OFF_MEAS_CTRL);

    // tempo     . beg_loop   volume . end_loop X 4 channels
    // 0000_0000 . 0000_00  . 00 00    00_0000
    let meta = mem::get_sub_area(mem::LOC_HARD, mem::OFF_MEAS_META);

    // reserved . music playing . sfx playing (3, 2, 1, 0)
    // 000      . 0             . 0000
    let flag = mem::get_sub_area(mem::LOC_HARD, mem::OFF_CHAN_FLAG);

    // TODO: document this one too :).
    let notes = mem::get_sub_area(mem::LOC_HARD, mem::OFF_NOTES);

    // TODO: document this. Really, just array of notes.
    let meas = mem::get_sub_area(mem::LOC_HARD, mem::OFF_MEAS_DATA);

    for i in 0..4 {
        let channel_one_mask = 0b0000_0001 << i;
        let channel_zer_mask = !(0b0000_0001 << i);
        assert!(channel_one_mask < 0b0001_0000);

        // skip this channel if it isn't playing a sound effect.
        if flag[0] & channel_one_mask == 0 { continue; }

        let (note_ind, time_ind) = (i*2, i*2+1);
        let (note_p1, note_p2) = (i*4+2, i*4+3);
        let time_left  = ctrl[time_ind];
        let tempo      = meta[i*cart::SIZ_MEASURE_META];

        let (m1, m2)   = (meta[i*cart::SIZ_MEASURE_META+1], meta[i*cart::SIZ_MEASURE_META+2]);
        let beg_loop = m1 & 0b1111_1100 >> 2;
        let volume   = m1 & (0b0000_0011 << 2) | (m2 & 0b1100_0000 >> 6);
        let end_loop = m2 & 0b0011_1111;

        // assert for logic above
        assert!(beg_loop < 64);
        assert!(end_loop < 64);
        assert!(volume < 16);

        // if time left is zero, then do something.
        if time_left > 0 {
            assert!(ctrl[time_ind] != 0);
            ctrl[time_ind] -= 1;
        } else {
            assert!(ctrl[time_ind] == 0 && time_left == 0);

            let note_index = ctrl[note_ind] & 0b0011_1111;
            assert!(note_index <= 63);

            if end_loop > beg_loop {
                assert!(end_loop >= 1);
                assert!(end_loop <= 63);
                // minus 1, because the end loop is exclusive
                ctrl[time_ind] = tempo;
                if note_index >= end_loop-1 {
                    assert!(beg_loop <= 63);
                    ctrl[note_ind] = beg_loop;
                } else {
                    // TODO: think about this a bit more. Will it ever reach the end of the loop?
                    ctrl[note_ind] = ((ctrl[note_ind] & 0b0011_1111) +  1) & 0b0011_1111;
                }

                // TODO: FIX DUPLICATE CODE
                let note_index = (ctrl[note_ind] & 0b0011_1111) as usize;
                if note_index >= 32 {
                    notes[note_p1] = 0;
                    notes[note_p2] = 0;
                } else {
                    assert!(note_index < 32);
                    notes[note_p1] = meas[i*64 + note_index*2];
                    notes[note_p2] = meas[i*64 + note_index*2+1];
                }
            } else if note_index >= 31 { // should actually never be greater than.
                // if finished, then we can switch the sound effect thing to inactive.
                flag[0] &= channel_zer_mask;

                // clear the note actually playing.
                notes[note_p1] = 0;
                notes[note_p2] = 0;
            } else  {
                ctrl[time_ind] = tempo;
                ctrl[note_ind] += 1;

                // TODO: FIX DUPLICATE CODE
                let note_index = ctrl[note_ind] & 0b0011_1111;
                assert!(note_index <= 31);
                notes[note_p1] = meas[i*64 + note_index as usize *2];
                notes[note_p2] = meas[i*64 + note_index as usize *2+1];
            }
        }
    }
}

// music
pub fn play_music() {

}

// <no params>
pub fn pause_music() {

}

// <no params>
pub fn resume_music() {

}

// If variables are out of bounds, do NOTHING.
// channel = 4, find available channel (or channel 0 if no available channel found.
pub fn play_measure(sfx: usize, mut channel: usize) {
    if channel > 5 { return; }
    let sfx_loc = cart::get_measure_data_loc(sfx);
    let sfx_meta_loc = cart::get_measure_meta_loc(sfx);
    if sfx_loc == mem::LOC_NULL { return; }
    assert!(sfx_loc.end - sfx_loc.start > 0);
    assert!(sfx_meta_loc.end - sfx_meta_loc.start > 0);

    let flag = mem::get_sub_area(mem::LOC_HARD, mem::OFF_CHAN_FLAG);

    // Find an available channel.
    if channel == 4 {
        channel = 0; // if not found, make 0.
        for i in 0..4 {
            let loc = 0b0000_0001 << i;
            if (loc & flag[0]) == 0 {
                channel = i;
                break;
            }
        }
    }

    assert!(channel < 4);

    // Copy sound effecj.
    mem::mcpy_w(mem::LOC_HARD.start + mem::OFF_MEAS_DATA.start + cart::SIZ_MEASURE_DATA*channel, sfx_loc.start, cart::SIZ_MEASURE_DATA);
    mem::mcpy_w(mem::LOC_HARD.start + mem::OFF_MEAS_META.start + cart::SIZ_MEASURE_META*channel, sfx_meta_loc.start, cart::SIZ_MEASURE_META);

    // Reset temp data
    let ctrl = mem::get_sub_area(mem::LOC_HARD, mem::OFF_MEAS_CTRL);
    ctrl[channel*2] = 0;
    ctrl[channel*2+1] = mem::peek(mem::OFF_MEAS_META.start + channel*cart::SIZ_MEASURE_META);

    // Enable channel
    flag[0] = flag[0] | (0b0000_0001 << channel);
}

// channel
pub fn pause_measure(channel: usize) {
    // TODO: when paused, make it stop playing the note.
    if channel >= 4 { return; }
    // just need to un-enable it
    let flag = mem::get_sub_area(mem::LOC_HARD, mem::OFF_CHAN_FLAG);
    flag[0] = flag[0] & !(0b0000_0001 << channel);
}

// channel
pub fn resume_measure(channel: usize) {
    // TODO: when resumed, make it play the note again.
    if channel >= 4 { return; }
    // just need to enable it :)
    let flag = mem::get_sub_area(mem::LOC_HARD, mem::OFF_CHAN_FLAG);
    flag[0] = flag[0] | 0b0000_0001 << channel;
}
