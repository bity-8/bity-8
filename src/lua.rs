// this is the lua module
// RIGHT NOW, IT IS VERY MESSY!!!
extern crate hlua;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;

// loads and evaluates the lua file.
// 
pub fn load_file(file: &Path) -> hlua::Lua {
    // Create lua virtual machine.
    let mut lua = hlua::Lua::new();

    let f = File::open(file).expect("file not found");

    // The reader must read from a file, and avoids putting the file into memory.
    // If we are okay putting the file into memory as a string first, we could just use the execute
    // function.
    let buf_reader = BufReader::new(f);
    let read_res = lua.execute_from_reader::<(), _>(buf_reader);

    if let Err(e) = read_res {
        match e {
            hlua::LuaError::SyntaxError(s) => eprintln!("Error: {:?}", s),
            _ => eprintln!("Error: {:?}", e)
        }
    }

    lua
}

fn call_fn() {

}

pub fn call_init(mut lua: hlua::Lua) -> Result {
    match lua.execute::<()>("_init()") {
        Ok(_v) => (),
        Err(_e) => eprintln!("Error: _init() not found"),
    }
}

pub fn call_update(mut lua: hlua::Lua) {
    match lua.execute::<()>("_update()") {
        Ok(_v) => (),
        Err(_e) => eprintln!("Error: _update() not found"),
    }
}

pub fn call_draw(mut lua: hlua::Lua) {
    match lua.execute::<()>("_draw()") {
        Ok(_v) => (),
        Err(_e) => eprintln!("Error: _draw() not found"),
    }
}
