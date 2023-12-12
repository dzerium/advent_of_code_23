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

    let sum = trebuchet(content);
    println!("sum : {}", sum);
    wait_exit();
}

fn replace (line: &str) -> String {
    line.to_lowercase()
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
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

fn trebuchet(content: String) -> u32 {
    let mut sum: u32 = 0;
    let transed = replace(&content);

    for (_i, line) in transed.lines().enumerate() {
        sum += get_number(line);
    }
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