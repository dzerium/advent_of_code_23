use regex::Regex;
use sscanf::scanf;

#[derive(Debug)]
pub struct Record {
    number: u32,
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
    matches: u32,
    copies: u32,
}

impl Record {
    fn new (number: u32, winning_numbers: &str, own_numbers: &str) -> Record {
        let re = Regex::new(r"\d+").unwrap();

        let winning_numbers: Vec<u32> = re.find_iter(winning_numbers).filter_map(|x| x.as_str().parse().ok()).collect();
        let own_numbers: Vec<u32> = re.find_iter(own_numbers).filter_map(|x| x.as_str().parse().ok()).collect();
        let matches = own_numbers.iter().filter(|x| winning_numbers.contains(x)).collect::<Vec<&u32>>().len() as u32;

		Record {
			number,
			winning_numbers,
			own_numbers,
			matches,
			copies: 1,
		}
	}
}


pub fn get_card (content: String) -> Vec::<Record> {
    let mut records = Vec::<Record>::new();

    content
        .lines()
        .into_iter()
        .for_each(| line | {
            let parsed = scanf!(line.trim(), "Card{}{}:{}|{}", str, u32,str,str);
            println!("{:?}", parsed);
            if let Ok((_spaces, number, winning_numbers, own_numbers)) = parsed {
                let record = Record::new(number, winning_numbers, own_numbers);
                records.push(record);
			}
        });
    records
}

pub fn get_worth (record: &Vec<Record>) -> u32 {
    let mut sum: u32 = 0;
    for r in record {
        let point = if r.matches > 1 { (2 as u32).pow(r.matches - 1) } else { r.matches };
        sum += point;
	}
    sum
}

pub fn get_total_copies (mut records: Vec<Record>) -> u32 {
    let mut i: usize = 0; 

    while i < records.len() {
        if (records[i].matches > 0) {
            let mut j :usize = 1;
		    while j <= (records[i].matches) as usize && i + j < records.len(){
                records[i + j].copies += records[i].copies;
		        j += 1;
            }
        }
		i += 1;
	}
	records.iter().map(|x| x.copies).sum()
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

    let mut records = get_card(input.to_string());
    assert!(get_worth(&records) == 13);
    println!("{}", get_total_copies(records));
}
