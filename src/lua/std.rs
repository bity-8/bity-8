extern crate hlua;

use memory;

pub fn load_std(lua: &mut hlua::Lua) {
    lua.openlibs(); // this is for testing, delete when not needed.
    lua.set("peek", hlua::function1(|loc: i32|      memory::peek(loc as usize)));
    lua.set("poke", hlua::function2(|loc: i32, val| memory::poke(loc as usize, val)));
}

#[test]
fn test_peek_poke() {
    use lua;
    let mut l = lua::create_lua();
    l.execute::<()>("
        poke(1, -10)
        poke(2, 001)
        poke(8, 120)
        poke(10, 0x20)
        poke(189, -3020)

        for i=0, 200 do
           local val = peek(i)
           if val ~= 0 then
              print(\"val \"..i..\" is \"..val)
           end
        end").unwrap();
}
