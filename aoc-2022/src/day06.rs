use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    for (index, c) in input.chars().enumerate() {
        if index < 3 {
            continue;
        }
        let range = &input[index-3..index+1];
        let num_unique_last_4 = range.chars().fold(HashSet::new(), |mut acc, el| {
            acc.insert(el);
            acc
        }).len();

        if num_unique_last_4 == 4 {
            return index + 1;
        }
    }
    0
}

pub fn part2(input: &str) -> usize {
    for (index, c) in input.chars().enumerate() {
        if index < 13 {
            continue;
        }
        let range = &input[index-13..index+1];
        let num_unique_last_14 = range.chars().fold(HashSet::new(), |mut acc, el| {
            acc.insert(el);
            acc
        }).len();

        if num_unique_last_14 == 14 {
            return index + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_soln("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1_soln("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1_soln("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1_soln("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        assert_eq!(part1("input/2022/day6.txt"), 11);
        
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/2022/day6.txt"), 2980);
    }
}