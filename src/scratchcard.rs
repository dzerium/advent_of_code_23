use regex::Regex;
use sscanf::scanf;

#[derive(Debug)]
pub struct Record {
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
}

pub fn get_card(content: String) -> Vec<Record> {
    let mut record = Vec::<Record>::new();
    let re = Regex::new(r"\d+").unwrap();

    content.lines().into_iter().for_each(|line| {
        let parsed = scanf!(line.trim(), "Card{}{}:{}|{}", str, u32, str, str);
        println!("{:?}", parsed);
        if let Ok((_spaces, _number, winning_numbers, own_numbers)) = parsed {
            let winning_numbers: Vec<u32> = re
                .find_iter(winning_numbers)
                .filter_map(|x| x.as_str().parse().ok())
                .collect();
            let own_numbers: Vec<u32> = re
                .find_iter(own_numbers)
                .filter_map(|x| x.as_str().parse().ok())
                .collect();

            record.push(Record {
                winning_numbers,
                own_numbers,
            });
        }
    });
    record
}

pub fn get_worth(record: &Vec<Record>) -> u32 {
    let mut sum: u32 = 0;
    for r in record {
        let paired: Vec<&u32> = r
            .own_numbers
            .iter()
            .filter(|x| r.winning_numbers.contains(x))
            .collect::<Vec<&u32>>();
        let count = paired.len() as u32;

        let point = if count > 1 {
            (2 as u32).pow(count - 1)
        } else {
            count
        };
        sum += point;
    }
    sum
}

#[test]
fn recursion() {
    let input = r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#
    .trim();

    let record = get_card(input.to_string());
    println!("{}", get_worth(&record));
}
