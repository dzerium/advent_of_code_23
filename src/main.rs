mod cube_conondrum;
mod trebuchet;
mod utils;

use cube_conondrum::*;
use std::env;
use std::io;
use trebuchet::*;
use utils::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let content = match file::read_file("../../input/2_cube.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Debug info: {:?}", e);
            return;
        }
    };

    let records = cube_conondrum(content);
    let (sum, pow) = validate_records(&records);
    println!("sum : {}, pow {}", sum, pow);
    wait_exit();
}

fn wait_exit() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Press any key to exit");
    stdin.read_line(&mut buffer).expect("Cannot read line");
}
