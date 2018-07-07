extern crate hlua;
extern crate bresenham;

use self::bresenham::Bresenham;
use memory as mem;
use std::cmp;

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
    lua.set("rand", hlua::function1(|upper: f32| -> f32            {4f32}));
    
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
      // let mut theta = 0f32;
      // let step = 0.05;

      // while theta <= 360f32 {
      //   let x2 = x as f32 + (radius as f32 * theta.cos());
      //   let y2 = y as f32 + (radius as f32 * theta.sin());
      //   draw_line(x,y,x2 as i32,y2 as i32,color);
      //   theta += step;
      // for y in -radius..radius {
      //   for x in -radius..radius {
      //     if x*x + y*y <= radius*radius {
      //       set_point(x0 + x, y0 + y, color);
      //     }
      //   }
      // }

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

    // Input, for this, the integer type shouldn't matter
    // In fact, maybe (not sure), all integers should be 32 bit for the std functions.
    lua.set("btn_reg", hlua::function0(|| -> i32 {
      mem::peek(mem::LOC_HARD.start + mem::OFF_INPUT.start) as i32
    }));

    lua.set("btn", hlua::function1(|button: i32| -> bool {
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
  let x = cmp::min(192, x);
  let y = cmp::min(144, y);
  mem::LOC_SCRE.start + x as usize /2 + (192/2 * y as usize)
}

fn draw_line(x1:i32,y1:i32,x2:i32,y2:i32,color:u8) {
  for (x, y) in Bresenham::new((x1 as isize,y1 as isize),(x2 as isize,y2 as isize)) {
        if (x < 0 || x > 192) || (y < 0 || y > 144) {
          continue;
        }
        set_point(x as i32,y as i32,color);
      }
}

fn in_bounds(x:i32, y:i32) -> bool {
  x >= 0 && x < 192 && y >= 0 && y < 144
}

fn draw_horiz_line(x1:i32,x2:i32,y:i32,color:u8) {
  let mut x_min = cmp::max(cmp::min(x1, x2), 0);
  let mut x_max = cmp::min(cmp::max(x1, x2), 193);
  if x_min < 0 || x_max > 192 || y < 0 || y > 144 {
    return;
  }
  let length = x_min - x_max;
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
  let length = f32::ceil(x_max as f32/2.0 - x_min as f32/2.0) as usize;
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
