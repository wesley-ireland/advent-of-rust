use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn part1(file: &str) -> String {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);

    let lines = reader.lines()
        .fold(Vec::new(), |mut lines_vec, line| {
            match line {
                Ok(line) => lines_vec.push(line),
                Err(_) => {}
            }
            lines_vec
        });

    lines.iter().for_each(|line| {
        println!("{}, len: {}", line, line.len());
    });

    let (stacks_input, commands) = read_input(lines);
    let stacks = build_stacks(stacks_input);
    let stacks = operate_crane(stacks, commands);
    print_tops(&stacks)
}

pub fn part2(file: &str) -> String {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);

    let lines = reader.lines()
        .fold(Vec::new(), |mut lines_vec, line| {
            match line {
                Ok(line) => lines_vec.push(line),
                Err(_) => {}
            }
            lines_vec
        });

    lines.iter().for_each(|line| {
        println!("{}, len: {}", line, line.len());
    });

    let (stacks_input, commands) = read_input(lines);
    let stacks = build_stacks(stacks_input);
    let stacks = operate_crane_9001(stacks, commands);
    print_tops(&stacks)
}

fn print_tops(stacks: &Vec<Vec<char>>) -> String {
    let mut tops = String::new();
    for stack in stacks {
        tops.push(stack[stack.len()-1]);
    }
    tops
}

fn read_input(input: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut split_index = 0;
    for (index, line) in input.iter().enumerate() {
        if line == " 1   2   3   4   5   6   7   8   9 " {
            split_index = index;
        }
    }
    
    let stacks = &input[0..split_index].to_vec();
    let commands = &input[split_index+2..input.len()].to_vec();

    (stacks.to_vec(), commands.to_vec())
}

fn build_stacks(input: Vec<String>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec!(
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new()
    );
    
    for line in input.iter().rev() {
        let item1 = line.chars().nth(1).unwrap();
        let item2 = line.chars().nth(5).unwrap();
        let item3 = line.chars().nth(9).unwrap();
        let item4 = line.chars().nth(13).unwrap();
        let item5 = line.chars().nth(17).unwrap();
        let item6 = line.chars().nth(21).unwrap();
        let item7 = line.chars().nth(25).unwrap();
        let item8 = line.chars().nth(29).unwrap();
        let item9 = line.chars().nth(33).unwrap();
        
        if item1 != ' ' {
            stacks[0].push(item1);
        }
        if item2 != ' ' {
            stacks[1].push(item2);
        }
        if item3 != ' ' {
            stacks[2].push(item3);
        }
        if item4 != ' ' {
            stacks[3].push(item4);
        }
        if item5 != ' ' {
            stacks[4].push(item5);
        }
        if item6 != ' ' {
            stacks[5].push(item6);
        }
        if item7 != ' ' {
            stacks[6].push(item7);
        }
        if item8 != ' ' {
            stacks[7].push(item8);
        }
        if item9 != ' ' {
            stacks[8].push(item9);
        }
    }
    
    stacks
}

fn parse_command(command: &str) -> (usize, usize, usize) {
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let captures = regex.captures(&command).unwrap();
    
    let amount = captures[1].parse::<usize>().unwrap();
    let from_stack_index = captures[2].parse::<usize>().unwrap() - 1;
    let to_stack_index = captures[3].parse::<usize>().unwrap() - 1;
    
    (amount, from_stack_index, to_stack_index)
}

fn operate_crane(mut stacks: Vec<Vec<char>>, commands: Vec<String>) -> Vec<Vec<char>> {
    for command in commands {
        let (amount, from_stack_index, to_stack_index) = parse_command(&command);
        
        for _ in 0..amount {
            let crate_to_move = {
                stacks[from_stack_index].pop()
            };
            match crate_to_move {
                None => {}
                Some(val) => stacks[to_stack_index].push(val)
            }
        }
    }
    stacks
}

fn operate_crane_9001(mut stacks: Vec<Vec<char>>, commands: Vec<String>) -> Vec<Vec<char>> {
    for command in commands {
        let (amount, from_stack_index, to_stack_index) = parse_command(&command);

        let mut multi_crate = Vec::new();
        for _ in 0..amount {
            let crate_to_move = {
                stacks[from_stack_index].pop()
            };
            match crate_to_move {
                None => {}
                Some(val) => multi_crate.push(val)
            }
        }
        for _ in 0..amount {
            let crate_to_move = {
                multi_crate.pop()
            };
            match crate_to_move {
                None => {}
                Some(val) => stacks[to_stack_index].push(val)
            }
        }
    }
    stacks
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/2022/day5.txt"), "RLFNRTNFB");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/2022/day5.txt"), "MHQTLJRLB");
    }
}