extern crate hlua;
extern crate bresenham;

use self::bresenham::Bresenham;
use memory as mem;

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
    lua.set("draw_rect", hlua::function5(|x: i32, y: i32, width: i32, height: i32, color: i8| {
        let mut realwidth = width/2;
        let mut realheight = height;
        if x + realwidth >= 192 {
            realwidth = 192/2 - x/2;
        }
        if y + realheight >= 144 {
            realheight = 144 - y;
        }
        if realheight == 0 || realwidth == 0 { return; };
        for i in y..(y+realheight) {
            // Calculation:
            // 0x40400 (buffer addr)
            // x/2 (pixels are stored in nibbles)
            // 192/2 * i (calculate row offset)
            mem::mset_w(get_buffer_loc(x as isize,i as isize), realwidth as usize, color | (color << 4));
        }
    }));

    lua.set("draw_line", hlua::function5(|x1: i32, y1: i32, x2: i32, y2: i32, color: u8| {
      for (x, y) in Bresenham::new((x1 as isize,y1 as isize),(x2 as isize,y2 as isize)) {
        if (x < 0 || x > 192) || (y < 0 || y > 144) {
          continue;
        }
        let mut pixel_current = mem::peek(get_buffer_loc(x,y)) as u8;
        if (x & 1) == 0 {
          pixel_current = (pixel_current & 0x0F) | (color << 4);
        } else {
          pixel_current = (pixel_current & 0xF0) | color;
        }
        mem::poke_w(get_buffer_loc(x,y), pixel_current as i8);
      }
    }));

    //lua.set("_draw_circle", hlua::function4(|x: i32, y: i32, radius: i32, color: i8| {

    //}))

    // Input
    lua.set("btn_reg", hlua::function0(|| -> i8 {
      mem::peek(0x40031)
    }));
    lua.set("btn", hlua::function1(|button: i32| -> bool {
      let register = mem::peek(0x40031);
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
  (0x40400 + x/2 + (192/2 * y)) as usize
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
