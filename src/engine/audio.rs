extern crate sdl2;
extern crate rand;
use audio::rand::prelude::*;

use sdl2::audio::AudioSpecDesired;
use sdl2::audio::AudioQueue;
use self::sdl2::Sdl;
use memory as mem;

const CHANNELS:  u32 = 1;
const SAMPLES:   u32 = 1024;
const BITY_SAMP: f32 = 128f32;
const SPS:       u32 = 60;   // samples per second
const AMPLIFIER: i16 = 10;
const PIANO_LEN: usize = 88;

// Piano frequencies taken from here: A0 - C8
// http://www.sengpielaudio.com/calculator-notenames.htm
// Then rounded (with round)
const PIANO_FREQS_INT: [u32; PIANO_LEN] =
[  28,   29,   31,   33,   35,   37,   39,   41,   44,   46,   49,   52,
   55,   58,   62,   65,   69,   73,   78,   82,   87,   92,   98,  104,
  110,  117,  123,  131,  139,  147,  156,  165,  175,  185,  196,  208,
  220,  233,  247,  262,  277,  294,  311,  330,  349,  370,  392,  415,
  440,  466,  494,  523,  554,  587,  622,  659,  698,  740,  784,  831,
  880,  932,  988, 1046, 1109, 1175, 1245, 1319, 1397, 1480, 1568, 1661,
 1760, 1865, 1976, 2093, 2217, 2349, 2489, 2637, 2794, 2960, 3136, 3322,
 3520, 3729, 3951, 4186];


struct Channel {
    device: AudioQueue<i16>
}

// TODO: Clean this all up. (don't want a stupid static variable here.)
static mut INST_IND: usize = 0;
pub fn play_instrument(device: &mut AudioQueue<i16>, note: usize, wave: mem::MemLoc) {
    let wave_data = mem::get_area(wave);
    let period = ((SPS * SAMPLES) / PIANO_FREQS_INT[note % PIANO_LEN]) as f32;
    let mut result = Vec::new();

    // next: cache the last index.
    for x in 0..SAMPLES {
        let ind = (x as f32 % period / period * BITY_SAMP as f32) as usize;
        let amp = unsafe { wave_data[(ind + INST_IND) % 128] };
        let amp = if amp == -128 {
            let r: i8 = random();
            if r == -128 { -127 } else { r }
        } else { amp };
        //println!("{}", ind);

        result.push(amp as i16 * AMPLIFIER);
    }

    unsafe { INST_IND += (SAMPLES as f32 % period / period * BITY_SAMP as f32) as usize; }
    unsafe { INST_IND %= 128usize; }

    device.queue(&result);
}

pub fn init(sdl_context: &mut Sdl) -> AudioQueue<i16> {
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some((SPS * SAMPLES) as i32), // 128 * 60 = 7_680
        channels: Some(CHANNELS as u8),
        // mono  -
        samples: Some(SAMPLES as u16)
        // default sample size
        };

    let device = audio_subsystem.open_queue::<i16, _>(None, &desired_spec).unwrap();
    println!("{:?}", device.spec());
    device
}
