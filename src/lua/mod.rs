extern crate hlua;
pub mod std;

use self::hlua::Lua;
use self::hlua::LuaError;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

// loads and evaluates the lua file.
pub fn create_lua<'a>() -> Lua<'a> {
    let mut lua = Lua::new();
    std::load_std(&mut lua);
    lua
}

pub fn load_file(file: &Path, lua: &mut Lua) {
    // Create lua virtual machine.
    let f = File::open(file).expect("file not found");

    // The reader must read from a file, and avoids putting the file into memory.
    // If we are okay putting the file into memory as a string first, we could just use the execute
    // function.
    let buf_reader = BufReader::new(f);

    let read_res = lua.execute_from_reader::<(), _>(buf_reader);

    if let Err(e) = read_res {
        match e {
            LuaError::SyntaxError(s) => eprintln!("Error: {:?}", s),
            _ => eprintln!("Error: {:?}", e)
        }
    }
}

pub fn call_fn(func_str: &str, lua: &mut Lua) -> Result<(), String> {
    match lua.execute::<()>(func_str) {
        Ok(_v) => Ok(_v),
        Err(_e) => Err(format!("Error: \'{}\' not found.", func_str)),
    }
}
