use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let file = File::open("resources/2021/day2.txt").expect("Error opening file");
    let reader = BufReader::new(&file);

    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;

    for (_index, command) in reader.lines().enumerate() {
        let command: String = command.unwrap();

        let mut command_parts = command.split(' ');
        let direction = command_parts.next();
        let amount = command_parts.next();

        let amount: i32 = match amount {
            Some(x) => x.parse().unwrap(),
            None => 0
        };

        match direction {
            Some("forward") => horizontal_position += amount,
            Some("up") => depth -= amount,
            Some("down") => depth += amount,
            None => println!("no command found"),
            _ => println!("unknown command: {}", direction.unwrap())
        }
    }

    println!("2021: Day 2, part 1: {}", depth * horizontal_position);
}

pub fn part2() {
    let file = File::open("resources/2021/day2.txt").expect("Error opening file");
    let reader = BufReader::new(&file);

    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;
    let mut aim: i32 = 0;

    for (_index, command) in reader.lines().enumerate() {
        let command: String = command.unwrap();

        let mut command_parts = command.split(' ');
        let direction = command_parts.next();
        let amount = command_parts.next();

        let amount: i32 = match amount {
            Some(x) => x.parse().unwrap(),
            None => 0
        };

        match direction {
            Some("forward") => {
                horizontal_position += amount;
                depth += aim * amount;
            },
            Some("up") => aim -= amount,
            Some("down") => aim += amount,
            None => println!("no command found"),
            _ => println!("unknown command: {}", direction.unwrap())
        }
    }

    println!("2021: Day 2, part 1: depth: {}, position: {}, final: {}", depth, horizontal_position, depth * horizontal_position);
}
