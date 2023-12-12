mod utils;
mod trebuchet;
mod cube_conondrum;

use std::env;
use std::io;
use utils::*;
use trebuchet::*;
use cube_conondrum::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let content = match file::read_file("../../input/2_cube.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Debug info: {:?}", e);
            return;
        }
    };

    let sum = cube_conondrum(content);
    println!("sum : {}", sum);
    wait_exit();
}


fn wait_exit() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Press any key to exit");
    stdin.read_line(&mut buffer).expect("Cannot read line");
}

