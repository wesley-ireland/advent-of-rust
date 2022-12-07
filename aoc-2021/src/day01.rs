use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let file = File::open("resources/2021/day1.txt").expect("Error opening file");
    let reader = BufReader::new(&file);

    let mut current: i32;
    let mut previous: Option<i32> = None;
    let mut count = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        current = line.parse().unwrap();

        match previous {
            None => {
                previous = Some(current);
                continue;
            }
            Some(prev) => {
                if current > prev {
                    count += 1;
                }
                previous = Some(current);
            }
        }
    }

    println!("2021: Day 1, part 1: {count}");
}

pub fn part2() {
    let file = File::open("resources/2021/day1.txt").expect("Error opening file");
    let reader = BufReader::new(&file);

    // Read the input into a Vector
    let mut input: Vec<usize> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let _ = &input.push(line.parse().unwrap());
    }

    // Create a windows vector
    let mut windows: Vec<usize> = Vec::new();
    for i in 0..input.len() {
        if i + 2 >= input.len() {
            break;
        }
        windows.push(input[i] + input[i + 1] + input[i + 2]);
    }

    // Iterate through the windows vector as we did in part 1
    let mut current: usize;
    let mut previous: Option<usize> = None;
    let mut count = 0;

    for i in 0..windows.len() {
        current = windows[i];
        match previous {
            None => {
                previous = Some(current);
                continue;
            }
            Some(prev) => {
                if current > prev {
                    count += 1;
                }
                previous = Some(current);
            }
        }
    }

    println!("2021: Day 1, part 2: {count}");
}
