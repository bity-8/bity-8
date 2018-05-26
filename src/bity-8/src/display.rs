extern crate sdl2;

use std::path::Path;
use sdl2::event::Event;
use sdl2::image::{LoadSurface, INIT_PNG, INIT_JPG};
use sdl2::keyboard::Keycode;
use sdl2::mouse::Cursor;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

pub fn run(png: &Path) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Cursor", 192*2, 144*2)
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();

    let surface = match Surface::from_file(png) {
        Ok(surface) => surface,
        Err(err)    => panic!("failed to load cursor image: {}", err)
    };
    let cursor = match Cursor::from_surface(surface, 0, 0) {
        Ok(cursor) => cursor,
        Err(err) => panic!("failed to load cursor: {}", err)
    };
    cursor.set();

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
