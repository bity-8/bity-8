extern crate sdl2;
extern crate hlua;

const SCR_X: u32 = 192;
const SCR_Y: u32 = 144;
const PIX_LEN: u32 = 4; // the size for each pixel.

use self::sdl2::event::Event;
use self::sdl2::render::WindowCanvas;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;
use memory;

// TODO: make this function more pretty!
pub fn draw_screen(canvas: &mut WindowCanvas) {
    let mut pal = memory::get_hard_pal();
    let mut colors = [Color::RGB(0, 255, 0); 16];

    for i in 0..16 {
        colors[i] = Color::RGB(pal[i*3] as u8, pal[i*3+1] as u8, pal[i*3+2] as u8)
    }

    {
        let screen = memory::get_scre();
        let mut draw_func = |col, x, y| {
            canvas.set_draw_color(col);
            canvas.fill_rect(Rect::new(x*PIX_LEN as i32, y*PIX_LEN as i32, PIX_LEN, PIX_LEN)).unwrap();
        };

        // remember there are 2 pixels in each byte.
        for i in 0..screen.len() {
            let cols  = screen[i] as usize;
            let left  = (cols & 0xF0) >> 4;
            let right = (cols & 0x0F);

            let i = (i as i32) * 2;
            let x = i % SCR_X as i32;
            let y = i / SCR_X as i32;

            draw_func(colors[left], x, y);
            draw_func(colors[right], x+1, y);
        }
    }

    canvas.present();
}

pub fn run(l: &mut hlua::Lua) {
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
                _ => {}
            }
        }

        canvas.clear();
        draw_screen(&mut canvas);
        l.execute::<()>("_update()");
    }
}
