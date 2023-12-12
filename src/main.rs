use std::env;
use std::io;
mod utils;

use utils::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    trebuchet("../../input/1_trebuchet.txt");


    let mut buffer = String::new();
    let stdin = io::stdin(); 
    println!("Press any key to exit");
    stdin.read_line(&mut buffer).expect("Cannot read line");
}

fn get_number (line: &str) -> u32 {
    let mut first = '\0';
    let mut last = '\0';
    for  (_i, c) in line.chars().enumerate() {
        if (c.is_digit(10)) && first == '\0'{
            first = c;
            last = c;
        }
        else if (c.is_digit(10)) {
			last = c;
		}
    }
    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
}

fn trebuchet (file: &str) {
    let content = match file::read_file(file) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading {}: {}", file, e);
            eprintln!("Debug info: {:?}", e);
            return;
        }
    };

    let mut sum  : u32 = 0;
    for (_i, line) in content.lines().enumerate() {
        sum += get_number(line);
    }
    println!("Trebuchet: {}", sum);
}