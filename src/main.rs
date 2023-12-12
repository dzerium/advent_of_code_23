mod gear_ratio;
mod utils;

use gear_ratio::*;
use std::env;
use std::io;
use utils::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let content = match file::read_file("../../input/3_gear.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Debug info: {:?}", e);
            return;
        }
    };

    let (schema, parts) = gear_ratio(content);
    let sum = validate_parts(&parts, &schema);
    println!("sum: {}", sum);
    wait_exit();
}

fn wait_exit() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Press any key to exit");
    stdin.read_line(&mut buffer).expect("Cannot read line");
}
