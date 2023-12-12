use std::env;
use std::io;
use regex::{RegexSet, Regex};

mod utils;

use utils::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

     let content = match file::read_file("../../input/1_trebuchet.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Debug info: {:?}", e);
            return;
        }
    };

    trebuchet(content);
    
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
	    result = numbers[0].1 * 10 + numbers[numbers.len() - 1].1;
    }
    result
}

fn trebuchet(content: String) -> u32 {
    let mut sum: u32 = 0;

    for (_i, line) in content.lines().enumerate() {

        let mut num_matches = regex_get_num(line);
        let mut word_matches = regex_words_to_num(line);

        num_matches.append(&mut word_matches);

        let total = get_number(num_matches.clone());
        sum += total;
        println!("{}: {:?} {:?} {} {}", line, num_matches, word_matches, total, sum);
    }
    println!("Trebuchet: {}", sum);
    sum
}

fn wait_exit() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Press any key to exit");
    stdin.read_line(&mut buffer).expect("Cannot read line");
}


#[test]
fn it_works_1() {
  let input = r#"
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
    "#.trim();

    let sum = trebuchet(input.to_string());
    assert_eq!(sum, 142);
}

#[test]
fn it_works_2() {
  let input = r#"
    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    "#.trim();

    let sum = trebuchet(input.to_string());
    assert_eq!(sum, 281);
}