fn replace(line: &str) -> String {
    line.to_lowercase()
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn get_number(line: &str) -> u32 {
    let mut first = '\0';
    let mut last = '\0';
    for (_i, c) in line.chars().enumerate() {
        if c.is_digit(10) && first == '\0' {
            first = c;
            last = c;
        } else if c.is_digit(10) {
            last = c;
        }
    }
    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
}

pub fn trebuchet(content: String) -> u32 {
    let mut sum: u32 = 0;
    let transed = replace(&content);

    for (_i, line) in transed.lines().enumerate() {
        sum += get_number(line);
    }
    sum
}


#[test]
fn it_works_1() {
    let input = r#"
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
    "#
    .trim();

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
    "#
    .trim();

    let sum = trebuchet(input.to_string());
    assert_eq!(sum, 281);
}

