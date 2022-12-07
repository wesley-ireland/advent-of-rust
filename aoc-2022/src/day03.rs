use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_priority(c: &char) -> i32 {
    let ascii = *c as i32;
    if c.is_uppercase() {
        ascii - 64 + 26
    } else {
        ascii - 96
    }
}

pub fn part1(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut priority_sum: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();
        let mut letter_set: HashSet<char> = HashSet::new();

        let compartment_one = &line[0..line.len() / 2];
        let compartment_two = &line[line.len() / 2..line.len()];

        for c in compartment_one.chars() {
            letter_set.insert(c);
        }
        for c in compartment_two.chars() {
            match letter_set.get(&c) {
                Some(_) => {
                    priority_sum += get_priority(&c);
                    break;
                },
                None => {}
            }
        }
    }
    return priority_sum;
}

fn find_key_for_value<'a>(map: &'a HashMap<i32, &'static str>, value: &str) -> Option<&'a i32> {
    map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None })
}

pub fn part2(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut priority_sum: i32 = 0;
    let mut group_letters: HashMap<char, i32> = HashMap::new();

    for (index, line) in lines.enumerate() {
        let line = line.unwrap();
        let line_letters: HashSet<char> = HashSet::from_iter(line.chars());

        group_letters = line_letters.iter().fold(group_letters.clone(), |mut map, c | {
            *map.entry(*c).or_insert(0) += 1;
            map
        });

        if (index + 1) % 3 == 0 {
            group_letters.iter().for_each(|kv| {

                let character = kv.0;
                let num_occurrences = kv.1;

                if *num_occurrences == 3 {
                    priority_sum += get_priority(&character);
                }
            });
            group_letters.drain();
        }
    }
    return priority_sum;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/2022/day3-example.txt"), 157);
        assert_eq!(part1("input/2022/day3.txt"), 8123);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/2022/day3-example.txt"), 70);
        assert_eq!(part2("input/2022/day3.txt"), 2620);
    }
}