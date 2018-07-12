extern crate hlua;
extern crate bresenham;

use self::bresenham::Bresenham;
use memory as mem;
use cartridge as cart;
use std::cmp;
use display;
use audio;

pub fn load_std(lua: &mut hlua::Lua) {
    lua.openlibs(); // this is for testing, delete when not needed.
    lua.set("_peek", hlua::function1(|pos: i32|                     mem::peek(  pos as usize)));
    lua.set("_poke", hlua::function2(|pos: i32, val|                mem::poke_w(pos as usize, val)));
    lua.set("_mset", hlua::function3(|pos: i32, len: i32, val|      mem::mset_w(pos as usize, len as usize, val)));
    lua.set("_mcpy", hlua::function3(|des: i32, pos: i32, len: i32| mem::mcpy_w(des as usize, pos as usize, len as usize)));
    // PICO-8 Math library: max, min, mid, floor, ceiling, cos, sin, atan2, sqrt, abs, rnd, srand
    lua.set("max" , hlua::function2(|val: f32, other: f32| -> f32  {f32::max(val, other)}));
    lua.set("min" , hlua::function2(|val: f32, other: f32| -> f32  {f32::min(val, other)}));
    lua.set("floor" , hlua::function1(|val: f32| -> f32            {f32::floor(val)}));
    lua.set("ceil" , hlua::function1(|val: f32| -> f32             {f32::ceil(val)}));
    lua.set("sin" , hlua::function1(|val: f32| -> f32              {f32::sin(val)}));
    lua.set("cos" , hlua::function1(|val: f32| -> f32              {f32::cos(val)}));
    lua.set("atan2", hlua::function2(|val1: f32, val2: f32| -> f32 {f32::atan2(val1, val2)}));
    lua.set("sqrt" , hlua::function1(|val: f32| -> f32             {f32::sqrt(val)}));
    lua.set("abs" , hlua::function1(|val: f32| -> f32              {f32::abs(val)}));
    lua.set("rand", hlua::function1(|_upper: f32| -> f32            {4f32}));

    // Some sound functions :)
    lua.set("play_sfx", hlua::function2(|sfx: i32, chan: i32| audio::play_measure(sfx as usize, chan as usize)));
    lua.set("pause_sfx", hlua::function1(|chan: i32| audio::pause_measure(chan as usize)));
    lua.set("resume_sfx", hlua::function1(|chan: i32| audio::resume_measure(chan as usize)));
    
    // PICO-8 Math bitwise: and, or, xor, not, rotl, rotr, left shift, right shift (arithmetic and logical)
    lua.set("band", hlua::function2(|val1: i32, val2: i32| -> i32   {val1 & val2}));
    lua.set("bor",  hlua::function2(|val1: i32, val2: i32| -> i32   {val1 & val2}));
    lua.set("bxor", hlua::function2(|val1: i32, val2: i32| -> i32   {val1 & val2}));
    lua.set("bnot", hlua::function1(|val1: i32| -> i32              {!val1}));
    //lua.set("_rotl", hlua::function2(|val: i32, amt: i32|))

    // Drawing
    lua.set("draw_rect", hlua::function5(|x: i32, y: i32, width: i32, height: i32, color: u8| {
        for i in y..(y+height) {
            draw_horiz_line(x, x + width, i, color);
        }
    }));

    lua.set("draw_line", hlua::function5(|x1: i32, y1: i32, x2: i32, y2: i32, color: u8| {
      if y1 == y2 {
        draw_horiz_line(x1, x2, y1, color);
      } else {
        draw_line(x1,y1,x2,y2,color);
      }
    }));

    lua.set("draw_dot", hlua::function3(|x:i32,y:i32,color:u8| {
      set_point(x,y,color);
    }));

    lua.set("draw_circle", hlua::function4(|x: i32, y: i32, radius: i32, color: u8| {
      // http://nand2tetris-questions-and-answers-forum.32033.n3.nabble.com/Fast-circle-algorithm-td4030808.html

      let mut i = 0;
      let mut j = radius;
      let mut counter = 3 - (radius + radius);

      draw_horiz_line(x - radius, x + radius, y, color);

      while j > i {
        if counter < 0 {
          counter = counter + 6 + i + i + i + i;
          i = i + 1;
        } else {
          if counter > 0 && j > i {
            j = j - 1;
            counter = (counter + 4) - (j + j + j + j);
          }
        }

        draw_horiz_line(x - i, x + i, y + j, color);
        draw_horiz_line(x - i, x + i, y - j, color);
        draw_horiz_line(x - j, x + j, y + i, color);
        draw_horiz_line(x - j, x + j, y - i, color);
        
      }
    }));
    lua.set("sprite", hlua::function5(|src_sheet:u32, src_x: u32, src_y: u32, x: i32, y: i32| {
      draw_sprite(src_sheet, src_x, src_y, x, y, 1);
    }));
    lua.set("sprite_t", hlua::function6(|src_sheet:u32, src_x: u32, src_y: u32, x: i32, y: i32, alpha: u8| {
      draw_sprite_transparent(src_sheet, src_x, src_y, x, y, 1, alpha);
    }));
    lua.set("load_tilemap", hlua::function2(|index: u32, dest: u32| {
      let tilemap_loc = cart::get_tile_map_loc(index as usize).start;
      let mem_loc = mem::LOC_TILE.start + (dest as usize * cart::SIZ_TILE_MAP);
      println!("{:x}", mem::peek(tilemap_loc));
      mem::mcpy_w(mem_loc, tilemap_loc, cart::SIZ_TILE_MAP);
    }));
    lua.set("load_sprites", hlua::function2(|index: u32, dest: u32| {
      let sprites_loc = cart::get_sprite_loc(index as usize).start;
      let mem_loc = mem::LOC_SPRI.start + (dest as usize * cart::SIZ_SPRITE);
      mem::mcpy_w(mem_loc, sprites_loc, cart::SIZ_SPRITE);
    }));
    lua.set("tilemap", hlua::function4(|map: u32, sheet: u32, screen_offset_x: i32, screen_offset_y: i32| {
      let tile_offset_x = (screen_offset_x.abs() >> 3) as usize;
      let tile_offset_y = (screen_offset_y.abs() >> 3) as usize;
      let num_sprites_x = {
        if (screen_offset_x.abs() & 7) != 0 {
          24 // This should be 25 once the scrambled sprites issue is fixed
        } else {
          24
        }
      };
      let num_sprites_y = {
        if (screen_offset_x.abs() & 7) != 0 {
          18 // This should be 19 once the scrambled sprites issue is fixed
        } else {
          18
        }
      };

      for y in 0..num_sprites_y {
        for x in 0..num_sprites_x {
          let tilemap_offset = mem::LOC_TILE.start + (map * 0x1B00) as usize + x + tile_offset_x + ((y + tile_offset_y) * 96);
          let tilemap_data = mem::peek(tilemap_offset) ;
          let spritesheet_x = tilemap_data >> 4;
          let spritesheet_y = tilemap_data & 15;
          let screen_x = (x << 3) as i32 - (screen_offset_x.abs() & 7) as i32; // Bitshifting math should be faster than multiplication and modulus division
          let screen_y = (y << 3) as i32 - (screen_offset_y.abs() & 7) as i32;
          //println!("Printing ({},{}) at ({},{})\tTilemap data: 0x{:x}\t", spritesheet_x, spritesheet_y, screen_x, screen_y, tilemap_data);
          draw_sprite(sheet, spritesheet_x.into(), spritesheet_y.into(), screen_x, screen_y, 1);
        }
      }
    }));
    lua.set("print", hlua::function5(|msg: String, x: i32, y: i32, fg: u8, bg: u8| {
      //println!("Printing: {}", msg);
      for (i, c) in msg.chars().enumerate() {
        if c as u8 <= 32 { continue; }
        else {
          if x + (i * 6) as i32 > 192 { continue; }
          let font_x = (c as u8 - 32) & 15;
          let font_y = (c as u8 - 32) >> 4;
          draw_font(font_x.into(), font_y.into(), x + (i * 6) as i32, y, fg, bg);
        }
      }
    }));

    // Input, for this, the integer type shouldn't matter
    // In fact, maybe (not sure), all integers should be 32 bit for the std functions.
    lua.set("btn_reg", hlua::function0(|| -> u8 {
      mem::peek(mem::LOC_HARD.start + mem::OFF_INPUT.start)
    }));

    lua.set("btn", hlua::function1(|button: u32| -> bool {
      let register = mem::peek(mem::LOC_HARD.start + mem::OFF_INPUT.start);
      match button {
        0 => (register & 0b00000001) > 0,
        1 => (register & 0b00000010) > 0,
        2 => (register & 0b00000100) > 0,
        3 => (register & 0b00001000) > 0,
        4 => (register & 0b00010000) > 0,
        5 => (register & 0b00100000) > 0,
        6 => (register & 0b01000000) > 0,
        7 => (register & 0b10000000) > 0,
        _ => false
      }
    }));
}

fn get_buffer_loc(x: isize, y: isize) -> usize{
  let x = cmp::min(192, x) as u32;
  let y = cmp::min(144, y) as u32;
  mem::LOC_SCRE.start + x as usize /2 + (display::SCR_X/2 * y) as usize
}

fn draw_line(x1:i32,y1:i32,x2:i32,y2:i32,color:u8) {
  for (x, y) in Bresenham::new((x1 as isize,y1 as isize),(x2 as isize,y2 as isize)) {
        if (x < 0 || x > display::SCR_X as isize) || (y < 0 || y > display::SCR_Y as isize) {
          continue;
        }
        set_point(x as i32,y as i32,color);
      }
}

fn draw_sprite(src_sheet: u32, src_x: u32, src_y: u32, x: i32, y: i32, size: u32) {
  let mut sprite_offset = mem::LOC_SPRI.start + (src_x as usize * 4) + (48 * src_y*8) as usize + (src_sheet * 0xD80) as usize;
  let start = cmp::max(cmp::min(y,0).abs(), 0);
  if (x & 1) == 1 {
    for i in start..8*size as i32 {
    for j in 0..4*size as i32 {
      let sprite_pixel = mem::peek(sprite_offset + j as usize + (48 * i) as usize);
      set_point(x + j * 2, y + i, sprite_pixel >> 4);
      set_point(x + j * 2 + 1, y + i, sprite_pixel & 15);
    }
  }
  } else {
    for i in start..8*size as i32 {
      if sprite_offset < mem::LOC_SCRE.start {
        println!("Line out of bounds");
        continue;
      }
      let length = (size*4) as i32 + cmp::min(x/2, 0);
      if length < (size*4) as i32 {
        mem::mcpy_w(get_buffer_loc(0 as isize,(y+i as i32) as isize), sprite_offset + (x.abs()/2) as usize, length as usize);
      } else {
        mem::mcpy_w(get_buffer_loc(x as isize,(y+i as i32) as isize), sprite_offset as usize, (size*4) as usize);
      }
      sprite_offset += 48;
    }
  }
}

fn draw_sprite_transparent(src_sheet: u32, src_x: u32, src_y: u32, x: i32, y: i32, size: u32, alpha: u8) {
  let sprite_offset = mem::LOC_SPRI.start + (src_x as usize * 4) + (48 * src_y*8) as usize + (src_sheet * 0xD80) as usize;
  let start = cmp::max(cmp::min(y,0).abs(), 0);

  for i in start..8*size as i32 {
    for j in 0..4*size as i32 {
      let sprite_pixel = mem::peek(sprite_offset + j as usize + (48 * i) as usize);
      if sprite_pixel >> 4 != alpha {
        set_point(x + j * 2, y + i, sprite_pixel >> 4);
      }
      if sprite_pixel & 15 != alpha {
        set_point(x + j * 2 + 1, y + i, sprite_pixel & 15);
      }
    }
  }
}

fn draw_font(src_x: u32, src_y: u32, x: i32, y: i32, fg: u8, bg: u8) {
  let sprite_offset = mem::LOC_FONT.start + (src_x as usize * 3) + (48 * src_y*6) as usize;
  let start = cmp::max(cmp::min(y,0).abs(), 0);
  //println!("{},{}", src_x, src_y);

  for i in start..6 as i32 {
    for j in 0..3 as i32 {
      let sprite_pixel = mem::peek(sprite_offset + j as usize + (48 * i) as usize);
      if sprite_pixel >> 4 == 0 && bg <= 15 {
        set_point(x + j * 2, y + i, bg);
      } else if sprite_pixel >> 4 == 1 && fg <= 15 {
        set_point(x + j * 2, y + i, fg)
      }
      if sprite_pixel & 15 == 0 && bg <= 15 {
        set_point(x + j * 2 + 1, y + i, bg);
      } else if sprite_pixel & 15 == 1 && fg <= 15 {
        set_point(x + j * 2 + 1, y + i, fg);
      }
    }
  }
}

fn in_bounds(x:i32, y:i32) -> bool {
  x >= 0 && x < display::SCR_X as i32 && y >= 0 && y < display::SCR_Y as i32
}

fn draw_horiz_line(x1:i32,x2:i32,y:i32,color:u8) {
  let mut x_min = cmp::max(cmp::min(x1, x2), 0);
  let x_max = cmp::min(cmp::max(x1, x2), 193);
  if x_min < 0 || x_max > display::SCR_X as i32 || y < 0 || y > display::SCR_Y as i32{
    return;
  }
  // This wasn't used, so I commented it (#nowarnings).
  // let _length = x_min - x_max;
  if (x_min & 1) == 1 {
    // Need to set right pixel in screen byte
    let mut pixel = mem::peek(get_buffer_loc(x_min as isize, y as isize));
    pixel = (pixel & 0xF0) | color;
    mem::poke_w(get_buffer_loc(x_min as isize, y as isize), pixel);
    x_min += 1;
  }
  if (x_max & 1) == 0 {
    // Need to set left pixel in screen byte
    let mut pixel = mem::peek(get_buffer_loc(x_max as isize, y as isize));
    pixel = (pixel & 0x0F) | (color << 4);
    mem::poke_w(get_buffer_loc(x_max as isize, y as isize), pixel);
  }
  let length = f32::ceil(x_max as f32/2.0 - x_min as f32/2.0);
  let length = {
    if length < 0.0 {
      0 as usize
    } else {
      length as usize
    }
  };
  mem::mset_w(get_buffer_loc(x_min as isize, y as isize), length, color | (color << 4));
}

fn set_point(x:i32,y:i32,color:u8) {
  if in_bounds(x,y) {
    let mut pixel_current = mem::peek(get_buffer_loc(x as isize,y as isize));
    if (x & 1) == 0 {
      pixel_current = (pixel_current & 0x0F) | (color << 4);
    } else {
      pixel_current = (pixel_current & 0xF0) | color;
    }
    mem::poke_w(get_buffer_loc(x as isize,y as isize), pixel_current);
  }
}

#[test]
fn test_peek_poke() {
    use lua;
    let mut l = lua::create_lua();
    l.execute::<()>("
        _poke(1, -10)
        _poke(2, 001)
        _poke(8, 120)
        _poke(10, 0x20)
        _poke(189, -3020)

        for i=0, 200 do
           local val = _peek(i)
           if val ~= 0 then
              print(\"val \"..i..\" is \"..val)
           end
        end").unwrap();
}