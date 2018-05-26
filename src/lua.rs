// this is the lua module
extern crate hlua;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;

// loads the file and runs any code in the init.
pub fn load_file(file: &Path) {
    // Create lua virtual machine.
    let mut lua = hlua::Lua::new();

    let f = File::open(file).expect("file not found");
    let buf_reader = BufReader::new(f);

    let read_res = lua.execute_from_reader::<(), _>(buf_reader);

    match read_res {

    }

    let val: i32 = lua.get("b").unwrap();
    println!("executed: {}", val);
}
