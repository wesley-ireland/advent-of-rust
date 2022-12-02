use std::fs::File;
use std::io::{BufRead, BufReader};

// A for Rock, B for Paper, and C for Scissors - opponent
// X for Rock, Y for Paper, and Z for Scissors - you

// 1 for Rock, 2 for Paper, and 3 for Scissors - you're score
// 0 if you lost, 3 if the round was a draw, and 6 if you won

// X beats C
// Y beats A
// Z beats B

pub fn part1(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut total_score: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();
        let throws: Vec<&str> = line.split(' ').collect();

        let opponent_throw = throws[0];
        let your_throw = throws[1];

        total_score += match your_throw {
            "X" => { // Rock 1
                match opponent_throw {
                    "A" => 4, // Rock, tie3
                    "B" => 1, // Paper, loss0
                    "C" => 7, // Scissors, win6
                    _ => {
                        println!("Opponent throw was invalid: {}", opponent_throw);
                        panic!();
                    }
                }
            }
            "Y" => { // Paper 2
                match opponent_throw {
                    "A" => 8, // Rock, win6
                    "B" => 5, // Paper, tie3
                    "C" => 2, // Scissors, loss0
                    _ => {
                        println!("Opponent throw was invalid: {}", opponent_throw);
                        panic!();
                    }
                }
            }
            "Z" => { // Scissors 3
                match opponent_throw {
                    "A" => 3, // Rock, loss0
                    "B" => 9, // Paper , win6
                    "C" => 6, // Scissors, tie3
                    _ => {
                        println!("Opponent throw was invalid: {}", opponent_throw);
                        panic!();
                    }
                }
            }
            _ => {
                println!("Your throw was invalid: {}", your_throw);
                panic!();
            }
        }
    }

    return total_score;
}

// A for Rock, B for Paper, and C for Scissors - opponent
// X for Lose, Y for Draw, and Z for Win - you

// 1 for Rock, 2 for Paper, and 3 for Scissors - you're score
// 0 if you lost, 3 if the round was a draw, and 6 if you won

// X beats C
// Y beats A
// Z beats B


pub fn part2(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut total_score: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();
        let throws: Vec<&str> = line.split(' ').collect();

        let opponent_throw = throws[0];
        let your_throw = throws[1];

        total_score += match your_throw {
            "X" => { // Lose 0
                match opponent_throw {
                    "A" => 3, // Rock, Scissors 3
                    "B" => 1, // Paper, Rock 1
                    "C" => 2, // Scissors, Paper 2
                    _ => {
                        println!("Opponent throw was invalid: {}", opponent_throw);
                        panic!();
                    }
                }
            }
            "Y" => { // Draw 3
                match opponent_throw {
                    "A" => 4, // Rock, Rock 1
                    "B" => 5, // Paper, Paper 2
                    "C" => 6, // Scissors, Scissors 3
                    _ => {
                        println!("Opponent throw was invalid: {}", opponent_throw);
                        panic!();
                    }
                }
            }
            "Z" => { // Win 6
                match opponent_throw {
                    "A" => 8, // Rock, Paper 2
                    "B" => 9, // Paper , Scissors 3
                    "C" => 7, // Scissors, Rock 1
                    _ => {
                        println!("Opponent throw was invalid: {}", opponent_throw);
                        panic!();
                    }
                }
            }
            _ => {
                println!("Your throw was invalid: {}", your_throw);
                panic!();
            }
        }
    }

    return total_score;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("resources/2022/day2-example.txt"), 15);
        assert_eq!(part1("resources/2022/day2.txt"), 14297);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("resources/2022/day2-example.txt"), 12);
        assert_eq!(part2("resources/2022/day2.txt"), 10498);
    }
}