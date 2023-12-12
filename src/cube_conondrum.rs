use sscanf;

#[derive(Debug)]
struct Colors(u8, u8, u8);

#[derive(Debug)]
struct GameRecord {
    id: usize,
    sets: Vec<Colors>,
}

impl GameRecord {
    fn new(id: usize) -> GameRecord {
        GameRecord {
            id,
            sets: Vec::new(),
        }
    }
}

const COLOR_LIMIT: Colors = Colors(12, 13, 14);

fn validate_records(records: Vec<GameRecord>) -> u32 {
    let mut sum = 0;
    for record in records {
        let mut is_valid = true;
        for set in record.sets {
            if set.0 > COLOR_LIMIT.0 || set.1 > COLOR_LIMIT.1 || set.2 > COLOR_LIMIT.2 {
                println!("Invalid set: {:?}", set);
                is_valid = false;
            }
        }
        if is_valid {
            sum += record.id as u32;
        }
    }
    sum
}

pub fn cube_conondrum(content: String) -> u32 {
    let mut records: Vec<GameRecord> = Vec::new();

    for (i, line) in content.lines().enumerate() {
        // We're sure that ids are continouos
        let mut record = GameRecord::new(i + 1);

        let sets = line.split(':').nth(1).expect("Should have sets").trim();
        let sets = sets.split(';').into_iter();

        for set in sets {
            let colors = set.split(',').into_iter();
            let mut r = 0u8;
            let mut g = 0u8;
            let mut b = 0u8;

            for color in colors {
                let (num, color) =
                    sscanf::scanf!(color.trim(), "{} {}", u8, String).expect("Should have color");

                // ! TODO: use match
                if (color == "red") {
                    r += num;
                } else if (color == "green") {
                    g += num;
                } else if (color == "blue") {
                    b += num;
                }
            }
            let color = Colors(r, g, b);
            record.sets.push(color);
        }
        records.push(record);
    }
    validate_records(records)
}

#[test]
fn cube_test_1() {
    let input = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#
    .trim();

    let sum = cube_conondrum(input.to_string());
    assert_eq!(sum, 8);
}
