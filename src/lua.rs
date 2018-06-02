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

fn call_fn(func_str: &str, mut lua: hlua::Lua) -> Result<(), String> {
    match lua.execute::<()>(func_str) {
        Ok(_v) => Ok(_v),
        Err(_e) => Err(format!("Error: \'{}\' not found.", func_str)),
    }
}

pub fn call_init(lua: hlua::Lua) -> Result<(), String> {
    call_fn("_init()", lua)
}

pub fn call_update(lua: hlua::Lua) -> Result<(), String> {
    call_fn("_update()", lua)
}

pub fn call_draw(lua: hlua::Lua) -> Result<(), String> {
    call_fn("_draw()", lua)
}
