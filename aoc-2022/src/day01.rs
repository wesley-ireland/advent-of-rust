use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut current_elf: i32 = 0;
    let mut max_elf: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();

        if line.is_empty() {
            if current_elf > max_elf {
                max_elf = current_elf;
            }
            current_elf = 0;
            continue;
        }

        current_elf += line.parse::<i32>().unwrap();
    }

    return max_elf;
}

pub fn part2(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut current_elf: i32 = 0;
    let mut max_elf1: i32 = 0;
    let mut max_elf2: i32 = 0;
    let mut max_elf3: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();
        
        println!("line {}", line);
        println!("current_elf {}", current_elf);
        println!("max_elf1 {}, max_elf2 {}, max_elf3 {}\n", max_elf1, max_elf2, max_elf3);

        if line.is_empty() {
            if current_elf > max_elf1 {
                max_elf3 = max_elf2;
                max_elf2 = max_elf1;
                max_elf1 = current_elf;
            } else if current_elf > max_elf2 {
                max_elf3 = max_elf2;
                max_elf2 = current_elf;
            } else if current_elf > max_elf3 {
                max_elf3 = current_elf;
            }
            current_elf = 0;
            continue;
        }

        current_elf += line.parse::<i32>().unwrap();
    }

    return max_elf1 + max_elf2 + max_elf3;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = part1("input/2022/day1-example.txt");
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part1() {
        let result = part1("input/2022/day1.txt");
        assert_eq!(result, 70698);
    }

    #[test]
    fn test_part2_example() {
        let result = part2("input/2022/day1-example.txt");
        assert_eq!(result, 45000);
    }

    #[test]
    fn test_part2() {
        let result = part2("input/2022/day1.txt");
        assert_eq!(result, 206643);
    }
}