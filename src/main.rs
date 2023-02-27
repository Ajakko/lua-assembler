use std::env;

mod lua;
use lua::LuaAssembler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} <path> <target-file-name>", &args[0]);
        ()
    }

    let files_path: &String = &args[1];
    let target_file_name: &String = &args[2];
    let mut compiler: LuaAssembler = LuaAssembler::new(target_file_name);

    if let Err(error) = compiler.add_files(files_path) {
        eprintln!("Error: could not add files from {files_path}\n{error}")
    };

    if let Err(error) = compiler.compile() {
        eprintln!("Error: could not compile to {target_file_name}\n{error}")
    };
}
