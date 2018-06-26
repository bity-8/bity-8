// TODO: take care of all the possible errors (unwraps).

extern crate sdl2;
extern crate rand;
use audio::rand::prelude::*;

use sdl2::audio::AudioSpecDesired;
use sdl2::audio::AudioQueue;
use self::sdl2::Sdl;
use memory as mem;

const SAMPLES:   u32 = 1024;
const BITY_SAMP: f32 = 128f32;
const SPS:       u32 = 60;   // samples per second
const AMPLIFIER: i16 = 10;
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
    mem::LOC_INS1, mem::LOC_INS2, mem::LOC_INS3, mem::LOC_INS4,
    mem::LOC_INS5, mem::LOC_INS6, mem::LOC_INS7, mem::LOC_INS8,
];

pub struct Channel {
    pub device: AudioQueue<i16>,
    pub current_index: usize,
}

impl Channel {
    pub fn play_note(&mut self, note: usize, wave_num: usize, volume: usize) {
        let volume = (volume % MAX_VOLUME) as i16;
        let wave_data = mem::get_area(INSTRUMENTS[wave_num % INSTRUMENTS_LEN].clone());
        let period = ((SPS * SAMPLES) / PIANO_FREQS_INT[note % PIANO_LEN]) as f32;
        let mut result = Vec::new();

        // next: cache the last index.
        for x in 0..SAMPLES {
            let ind = (x as f32 % period / period * BITY_SAMP as f32) as usize;
            let amp = wave_data[(ind + self.current_index) % 128];
            let amp = if amp == -128 {
                let r: i8 = random();
                if r == -128 { -127 } else { r }
            } else { amp };

            let amp = amp as i16 * AMPLIFIER / MAX_VOLUME as i16 * volume;
            result.push(amp as i16 * AMPLIFIER);
        }

        self.current_index += (SAMPLES as f32 % period / period * BITY_SAMP as f32) as usize;
        self.current_index %= 128usize;

        self.device.queue(&result);
    }

    pub fn new(sdl_context: &mut Sdl) -> Channel {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some((SPS * SAMPLES) as i32), // 128 * 60 = 7_680
            channels: Some(1u8),
            samples: Some(SAMPLES as u16)
        };

        let device = audio_subsystem.open_queue::<i16, _>(None, &desired_spec).unwrap();
        // println!("{:?}", device.spec());

        Channel {
            device: device,
            current_index: 0,
        }
    }
}
