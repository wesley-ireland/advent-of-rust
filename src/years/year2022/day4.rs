use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut num_complete_overlaps: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();
        let assignments: Vec<&str> = line.split(",").collect();
        let elf1: Vec<&str> = assignments[0].split("-").collect();
        let elf2: Vec<&str> = assignments[1].split("-").collect();
        let elf1_lower: i32 = elf1[0].parse().unwrap();
        let elf1_upper: i32 = elf1[1].parse().unwrap();
        let elf2_lower: i32 = elf2[0].parse().unwrap();
        let elf2_upper: i32 = elf2[1].parse().unwrap();

        if (elf1_lower <= elf2_lower && elf1_upper >= elf2_upper) ||
            (elf2_lower <= elf1_lower && elf2_upper >= elf1_upper) {
            num_complete_overlaps += 1;
        }
    }
    return num_complete_overlaps;
}


fn has_overlap(x: i32, y: i32, a: i32, b: i32) -> bool {
    !((x < a && x < b && y < a && y < b) || (x > a && x > b && y > a && y > b))
}

pub fn part2(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut num_complete_overlaps: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();
        let assignments: Vec<&str> = line.split(",").collect();
        let elf1: Vec<&str> = assignments[0].split("-").collect();
        let elf2: Vec<&str> = assignments[1].split("-").collect();

        let x: i32 = elf1[0].parse().unwrap();
        let y: i32 = elf1[1].parse().unwrap();
        let a: i32 = elf2[0].parse().unwrap();
        let b: i32 = elf2[1].parse().unwrap();

        if has_overlap(x, y, a, b) {
            num_complete_overlaps += 1;
        }
    }
    return num_complete_overlaps;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("resources/2022/day4-example.txt"), 2);
        assert_eq!(part1("resources/2022/day4.txt"), 507);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("resources/2022/day4-example.txt"), 4);
        assert_eq!(part2("resources/2022/day4.txt"), 897);
    }
}