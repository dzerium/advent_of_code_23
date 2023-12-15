mod scratchcard;
mod utils;

use scratchcard::*;
use std::env;
use std::io;
use utils::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let content = match file::read_file("../../input/4_scratch.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Debug info: {:?}", e);
            return;
        }
    };

    let record = get_card(content);
    println!("{}", get_worth(&record));
    println!("{}", get_total_copies(record));

    wait_exit();
}

fn wait_exit() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Press any key to exit");
    stdin.read_line(&mut buffer).expect("Cannot read line");
}
