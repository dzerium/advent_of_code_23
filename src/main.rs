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

    let mut sum : u64 = 0;

    for p in parts {
        if p.symbol != '*' || p.numbers.len() != 2 {
            continue
        } 
        let mut prod :u64 = 1; 
		for n in p.numbers {
            prod *= n.number as u64;
            print!("{:10}-", n.number);
        }
        sum += prod;
        println!("={:10} ** {:10}", prod, sum);

	}
    println!("{}", sum);

    wait_exit();
}

fn wait_exit() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Press any key to exit");
    stdin.read_line(&mut buffer).expect("Cannot read line");
}
