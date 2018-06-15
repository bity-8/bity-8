extern crate sdl2;

const SCR_X: u32 = 192;
const SCR_Y: u32 = 144;
const PIX_LEN: u32 = 2;

use self::sdl2::event::Event;
use self::sdl2::render::WindowCanvas;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;
use memory;

pub fn draw_screen(canvas: &mut WindowCanvas) {
    let mut pal = memory::get_hard_pal();
    let mut colors = [Color::RGB(0, 255, 0); 16];

    for i in 0..16 {
        colors[i] = Color::RGB(pal[i*3] as u8, pal[i*3+1] as u8, pal[i*3+2] as u8)
    }

    let screen = memory::get_scre();

    for i in 0..screen.len() {
        let x: i32 = i as i32*2 % SCR_X as i32;
        let y: i32 = i as i32*2 / SCR_X as i32;
        let left:  usize = (screen[i] as usize & 0xF0) >> 4;
        let right: usize = (screen[i] as usize & 0x0F);

        let col = colors[left];
        canvas.set_draw_color(col);
        canvas.fill_rect(Rect::new(x*PIX_LEN as i32, y*PIX_LEN as i32, PIX_LEN, PIX_LEN)).unwrap();

        let x: i32 = x + 1;
        let col = colors[right];
        canvas.set_draw_color(col);
        canvas.fill_rect(Rect::new(x*PIX_LEN as i32, y*PIX_LEN as i32, PIX_LEN, PIX_LEN)).unwrap();
    }
}

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Cursor", SCR_X*PIX_LEN, SCR_Y*PIX_LEN)
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                 Event::MouseButtonDown {x, y, ..} => {
                     draw_screen(&mut canvas);
                     canvas.present();
                }
                _ => {}
            }
        }
    }
}
