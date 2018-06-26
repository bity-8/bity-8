extern crate hlua;

use memory as mem;
use emulator;

pub fn load_std<'a, 'b> (em: &'b mut emulator::Emulator<'a>) {
    em.lua.openlibs(); // this is for testing, delete when not needed.
    //em.lua.set("_peek", hlua::function1(|pos: i32|                     em.mem.peek(  pos as usize)));
    //em.lua.set("_poke", hlua::function2(|pos: i32, val|                em.mem.poke_w(pos as usize, val)));
    //em.lua.set("_mset", hlua::function3(|pos: i32, len: i32, val|      em.mem.mset_w(pos as usize, len as usize, val)));
    //em.lua.set("_mcpy", hlua::function3(|des: i32, pos: i32, len: i32| em.mem.mcpy_w(des as usize, pos as usize, len as usize)));
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
