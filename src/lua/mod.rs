extern crate hlua;
pub mod std;

use self::hlua::Lua;
use self::hlua::LuaError;

// loads and evaluates the lua file.
pub fn create_lua<'a>() -> Lua<'a> {
    let mut lua = Lua::new();
    std::load_std(&mut lua);
    lua
}

pub fn load_code(code: &String, lua: &mut Lua) {
    // Create lua virtual machine.
    // The reader must read from a file, and avoids putting the file into memory.
    // If we are okay putting the file into memory as a string first, we could just use the execute
    // function.
    let result = lua.execute::<()>(code);

    if let Err(e) = result {
        match e {
            LuaError::SyntaxError(s) => eprintln!("Error: {:?}", s),
            _ => eprintln!("Error: {:?}", e)
        }
    }
}
