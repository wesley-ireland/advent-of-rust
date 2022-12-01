use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut currentElf: i32 = 0;
    let mut maxElf: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();

        if line.is_empty() {
            if currentElf > maxElf {
                maxElf = currentElf;
            }
            currentElf = 0;
            continue;
        }

        currentElf += line.parse::<i32>().unwrap();
    }

    return maxElf;
}

pub fn part2(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut currentElf: i32 = 0;
    let mut maxElf1: i32 = 0;
    let mut maxElf2: i32 = 0;
    let mut maxElf3: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();
        println!("line {}", line);
        println!("currentElf {}", currentElf);
        println!("maxElf1 {}, maxElf2 {}, maxElf3 {}\n", maxElf1, maxElf2, maxElf3);

        if line.is_empty() {
            if currentElf > maxElf1 {
                maxElf3 = maxElf2;
                maxElf2 = maxElf1;
                maxElf1 = currentElf;
            } else if currentElf > maxElf2 {
                maxElf3 = maxElf2;
                maxElf2 = currentElf;
            } else if currentElf > maxElf3 {
                maxElf3 = currentElf;
            }
            currentElf = 0;
            continue;
        }

        currentElf += line.parse::<i32>().unwrap();
    }

    return maxElf1 + maxElf2 + maxElf3;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = part1("resources/2022/day1-example.txt");
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part1() {
        let result = part1("resources/2022/day1.txt");
        assert_eq!(result, 70698);
    }

    #[test]
    fn test_part2_example() {
        let result = part2("resources/2022/day1-example.txt");
        assert_eq!(result, 45000);
    }

    #[test]
    fn test_part2() {
        let result = part2("resources/2022/day1.txt");
        assert_eq!(result, 206643);
    }
}