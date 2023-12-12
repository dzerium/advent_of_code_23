use std::env;
use std::io;
use regex::{RegexSet, Regex};

mod utils;

use utils::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    trebuchet("../../input/1_trebuchet.txt");
    
    wait_exit();
}


fn regex_words_to_num (line: &str) -> Vec<(usize, u32)> {
    let set = RegexSet::new(&[
        r"one",
        r"two",
        r"three",
        r"four",
        r"five",
        r"six",
        r"seven",
        r"eight",
        r"nine",
    ]).unwrap();

    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();

    let  matches: Vec<(usize, u32)> = set
        .matches(line)
        .into_iter()
        .map(|index| &regexes[index])
        .map(|re| {
            let m = re.find(line).unwrap().as_str();
            (
                line.find(m).unwrap(),  
                match m {
		            "one"   => 1,
		            "two"   => 2,
		            "three" => 3,
		            "four"  => 4,
		            "five"  => 5,
		            "six"   => 6,
		            "seven" => 7,
		            "eight" => 8,
		            "nine"  => 9,
		            _       => panic!("SHould not happen!")
                })
        })
        .collect();

     matches
}

fn regex_get_num(line: &str) -> Vec<(usize, u32)> {
    let re = Regex::new(r"[0-9]").unwrap();
    let mut idx_matches : Vec<(usize, u32)> = Vec::new();

    for cap in re.captures_iter(line) {
        let m = cap.get(0).unwrap().as_str();
        let number = m.parse::<u32>().unwrap();
        let idx = line.find(m).unwrap();

        idx_matches.push((idx, number));
    }

    idx_matches
}

fn get_number(mut numbers: Vec<(usize, u32)>) -> u32 {
    let mut result = 0;

    numbers.sort_by_key(|k| k.0);
    
    if !numbers.is_empty() {
	    //ret = first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
	    result = numbers[0].1 * 10 + numbers[numbers.len() - 1].1;
    }
    result
}

fn trebuchet(file: &str) {
    let content = match file::read_file(file) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading {}:) {}", file, e);
            eprintln!("Debug info: {:?}", e);
            return;
        }
    };

    let mut sum: u32 = 0;

    for (_i, line) in content.lines().enumerate() {

        let mut num_matches = regex_get_num(line);
        let mut word_matches = regex_words_to_num(line);

        num_matches.append(&mut word_matches);
        let total = get_number(num_matches.clone());
        sum += total;
        // println!("{}: {:?} {:?} {} {}", line, num_matches, word_matches, total, sum);
    }
    println!("Trebuchet: {}", sum);
}

fn wait_exit() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Press any key to exit");
    stdin.read_line(&mut buffer).expect("Cannot read line");
}


#[test]
fn it_works_9() {
    let idx_matches = regex_words_to_num("eightwo");
    println!("{:?}", idx_matches);
}

#[test]
fn it_works_10() {
    let mut num_matches = regex_get_num("eightwothree");
    let mut word_matches = regex_words_to_num("eightwothree");
    println!("{:?}", num_matches);
    println!("{:?}", word_matches);
    num_matches.append(&mut word_matches);
    let result = get_number(num_matches);
    assert_eq!(result, 83);
}