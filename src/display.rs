extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Cursor", 192*2, 144*2)
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
                     canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
                     canvas.fill_rect(Rect::new(x, y, 1, 1)).unwrap();
                     canvas.set_draw_color(Color::RGBA(0, 255, 255, 255));
                     canvas.fill_rect(Rect::new(x/2, y/2, 30, 30)).unwrap();
                     canvas.present();
                }
                _ => {}
            }
        }
    }
}
