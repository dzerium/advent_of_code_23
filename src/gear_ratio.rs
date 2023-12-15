use std::env;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Gear {
    pub symbol: char,
    x: usize,
    y: usize,
    pub numbers: Vec<Number>
}

#[derive(Debug, Clone)]
pub struct Number {
	x: usize,
	y: usize,
	pub number: u32,
}

impl Gear {
    fn new(symbol: char, x: usize, y: usize, numbers: Vec<Number>) -> Gear {
        Gear { symbol, x, y, numbers }
    }
}

fn get_number(schema: &Vec<Vec<char>>, x: usize, mut y: usize) -> Option<Number> {
    let max_len = schema[x].len();
    if !schema[x][y].is_numeric() {
        return None;
    }
    let start = loop {
        if y as isize - 1 < 0 || !schema[x][y - 1].is_numeric() {
			break y;
		}
        y -= 1;
    };

    let end = loop {
        if y + 1 == max_len || !schema[x][y + 1].is_numeric() {
            break y;
        }
        y += 1;
    };
    let number = Number{
       x: x,
       y: start,
       number: schema[x][start..=end].iter().collect::<String>().parse::<u32>().unwrap_or(0)
    };
    Some(number)
}

fn get_adjacent_numbers(schema: &Vec<Vec<char>>, x: usize, y: usize ) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];

    let max_x = if x + 1 < schema.len() {
        x + 1
    } else {
        x
    };
    let max_y = if y + 1 < schema[0].len(){
        y + 1
    } else {
        y
    };

    let min_x = if x - 1 > 0 { x - 1 } else { 0 };
    let min_y = if y - 1 > 0 { y - 1 } else { 0 };

    let mut i = min_x;
    
    //println!("[{}] {},{} ({},{}) ({},{})", schema[x][y], x, y, min_x, min_y, max_x, max_y);
    while i <= max_x {
        let mut j = min_y;
        while j <= max_y {
			//let number = get_number(&schema, i, j);
			//print!("{:?}", schema[i][j]);
            let num = get_number(&schema, i, j);
            if let Some(num) = num {
                if !numbers.iter().any(| n| n.x == num.x  && n.y == num.y) {
					numbers.push(num);
				} 
                
            }
			j += 1;
		}
        //println!();
        i += 1;
    }
	numbers
}

pub fn gear_ratio(content: String) -> (Vec<Vec<char>>, Vec<Gear>) {
    let mut schema = Vec::<Vec<char>>::new();
    let mut parts = Vec::<Gear>::new();

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

    for (i, sc) in schema.iter().enumerate() {
        let line = String::from_iter(sc);
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() || c == '.' {
                // do nothing
            } else {
                let numbers = get_adjacent_numbers(&schema, i, j);
                if numbers.len() > 0 {
					let gear = Gear::new(c, i, j, numbers);
                    println!("{:?}", gear);
					parts.push(gear);
                }
                
            }
        }
    }
    (schema, parts)
}

//#[test]
//fn gear_test_1() {
//    let input = r#"
//        467..114..
//        ...*......
//        ..35..633.
//        ......#...
//        617*......
//        .....+.58.
//        ..592.....
//        ......755.
//        ...$.*....
//        .664.598..
//        "#
//    .trim();

//    let (schema, parts) = gear_ratio(input.to_string());
   
//    //let sum = validate_parts(&parts, &schema);
//    //assert_eq!(sum, 4361);
//}

#[test]
fn recursion() {
    let input = r#"
.479........155..............944.....622..............31.........264.......................532..........................254.........528.....
..............-...............%.....+...................=....111*.................495.......+.......558..................../..........*.....
....................791*..62.....$.............847........&........-..........618.*...........818....&..642.........................789.....
....520.58......405......#....542.../587.............*....198.......846.........*..............*.......*....................647.............
.........*........./.964..........................474.302.....................786...43..............505..436...................*.....#51....
......832....@..........*.951*....984*111..801................../.....................-.......@............%.198......322.186...262.........
..........490........690......346.........................702&.566.%....................192...190.87............*.....-....=..%.........344.
....*.........................................816*588..............152..535................*.......*...........425...........53.............
..36.290.831....374................579.536.....................408.......*..733....998....169...146.....%179..........658...............260.
        "#
    .trim();

   let (schema, parts) = gear_ratio(input.to_string());    


    let mut sum : u64 = 0;

    for p in parts {
        if p.symbol != '*' || p.numbers.len() != 2 {
            continue
        } 
        let mut prod :u64 = 1; 
		for n in p.numbers {
            prod *= n.number as u64;
            print!("{}-", n.number);
        }
        sum += prod;
        println!("={} ** {}", prod, sum);

	}
    println!("{}", sum);

}