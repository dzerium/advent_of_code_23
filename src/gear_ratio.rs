use std::env;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Part {
    number: u32,
    x: usize,
    y: usize,
    len: usize,
}

impl Part {
    fn new(number: u32, x: usize, y: usize, len: usize) -> Part {
        Part { number, x, y, len }
    }
    fn is_valid(&self, schema: &Vec<Vec<char>>) -> bool {
        let schema_max_x = if schema[0].len() > 0 {
            schema[0].len() - 1
        } else {
            0
        };
        let schema_max_y = if schema.len() > 0 {
            schema.len() - 1
        } else {
            0
        };

        let min_x = if self.x > 0 { self.x - 1 } else { 0 };
        let max_x = if self.x + self.len < schema_max_x {
            self.x + self.len
        } else {
            schema_max_x
        };
        let min_y = if self.y > 0 { self.y - 1 } else { 0 };
        let max_y = if self.y + 1 < schema_max_y {
            self.y + 1
        } else {
            schema_max_y
        };
        let slices = &schema[min_y..=max_y];

        slices.iter().any(|slice| {
            let s = &slice[min_x..=max_x];
            s.iter().any(|c| !c.is_numeric() && *c != '.')
        })
    }
}


pub fn validate_parts(parts: &Vec<Part>, schema: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for part in parts {
        if part.is_valid(schema) {
            sum += part.number;
        }
    }
    sum
}

pub fn gear_ratio(content: String) -> (Vec<Vec<char>>, Vec<Part>) {
    let mut schema = Vec::<Vec<char>>::new();
    let mut parts = Vec::<Part>::new();

    env::set_var("RUST_BACKTRACE", "1");
    content
        .split('\n')
        .into_iter()
        .enumerate()
        .for_each(|(_i, line)| {
            let mut records: Vec<char> = Vec::new();
            line.trim().chars().enumerate().for_each(|(_j, c)| {
                records.push(c);
            });
            schema.push(records)
        });

    let re = Regex::new(r"(\d+)").unwrap();
    for (i, sc) in schema.iter().enumerate() {
        let line = String::from_iter(sc);

        for m in re.find_iter(&line) {
            let part = Part::new(
                m.as_str().parse::<u32>().unwrap(),
                m.start(),
                i,
                m.end() - m.start(),
            );
            parts.push(part);
        }
    }
    (schema, parts)
}

#[test]
fn gear_test_1() {
    let input = r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#
    .trim();

    let (schema, parts) = gear_ratio(input.to_string());
   
    let sum = validate_parts(&parts, &schema);
    assert_eq!(sum, 4361);
}
