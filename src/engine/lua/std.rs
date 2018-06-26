extern crate hlua;

use memory as mem;

pub fn load_std(lua: &mut hlua::Lua) {
    lua.openlibs(); // this is for testing, delete when not needed.
    lua.set("_peek", hlua::function1(|pos: i32|                     mem::peek(  pos as usize)));
    lua.set("_poke", hlua::function2(|pos: i32, val|                mem::poke_w(pos as usize, val)));
    lua.set("_mset", hlua::function3(|pos: i32, len: i32, val|      mem::mset_w(pos as usize, len as usize, val)));
    lua.set("_mcpy", hlua::function3(|des: i32, pos: i32, len: i32| mem::mcpy_w(des as usize, pos as usize, len as usize)));
    // PICO-8 Math library: max, min, mid, floor, ceiling, cos, sin, atan2, sqrt, abs, rnd, srand
    lua.set("_max" , hlua::function2(|val: f32, other: f32| -> f32  {f32::max(val, other)}));
    lua.set("_min" , hlua::function2(|val: f32, other: f32| -> f32  {f32::min(val, other)}));
    lua.set("_floor" , hlua::function1(|val: f32| -> f32            {f32::floor(val)}));
    lua.set("_ceil" , hlua::function1(|val: f32| -> f32             {f32::ceil(val)}));
    lua.set("_sin" , hlua::function1(|val: f32| -> f32              {f32::sin(val)}));
    lua.set("_cos" , hlua::function1(|val: f32| -> f32              {f32::cos(val)}));
    lua.set("_atan2", hlua::function2(|val1: f32, val2: f32| -> f32 {f32::atan2(val1, val2)}));
    lua.set("_sqrt" , hlua::function1(|val: f32| -> f32             {f32::sqrt(val)}));
    lua.set("_abs" , hlua::function1(|val: f32| -> f32              {f32::abs(val)}));
    lua.set("_rand", hlua::function1(|upper: f32| -> f32            {4f32}));
    
    // PICO-8 Math bitwise: and, or, xor, not, rotl, rotr, left shift, right shift (arithmetic and logical)
    lua.set("_and", hlua::function2(|val1: f32, val2: f32| -> f32   {f32::from_bits(val1.to_bits() & val2.to_bits())}));
    lua.set("_or",  hlua::function2(|val1: f32, val2: f32| -> f32   {f32::from_bits(val1.to_bits() | val2.to_bits())}));
    lua.set("_xor", hlua::function2(|val1: f32, val2: f32| -> f32   {f32::from_bits(val1.to_bits() ^ val2.to_bits())}));
    lua.set("_not", hlua::function1(|val1: f32| -> f32              {f32::from_bits(!(val1.to_bits()))}));


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
