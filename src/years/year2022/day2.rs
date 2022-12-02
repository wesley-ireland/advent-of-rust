use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::years::year2022::day2::YourThrow::{Paper, Rock, Scissors};

// A for Rock, B for Paper, and C for Scissors - opponent
// X for Rock, Y for Paper, and Z for Scissors - you

// 1 for Rock, 2 for Paper, and 3 for Scissors - you're score
// 0 if you lost, 3 if the round was a draw, and 6 if you won

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

enum OpponentThrow {
    Rock,
    Paper,
    Scissors
}

impl FromStr for OpponentThrow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(OpponentThrow::Rock),
            "B" => Ok(OpponentThrow::Paper),
            "C" => Ok(OpponentThrow::Scissors),
            _   => Err(())
        }
    }
}

enum YourThrow {
    Rock,
    Paper,
    Scissors
}

impl FromStr for YourThrow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(YourThrow::Rock),
            "Y" => Ok(YourThrow::Paper),
            "Z" => Ok(YourThrow::Scissors),
            _   => Err(())
        }
    }
}

struct RockPaperScissorsRoundPart1 {
    opponent_throw: OpponentThrow,
    your_throw: YourThrow
}

struct RockPaperScissorsRoundPart2 {
    opponent_throw: OpponentThrow,
    round_result: RoundResult
}

enum RoundResult {
    Lose,
    Draw,
    Win
}

trait RoundResultScore {
    fn score (&self) -> i32;
}

impl RoundResultScore for RoundResult {
    fn score(&self) -> i32 {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6
        }
    }
}

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _   => Err(())
        }
    }
}

trait ThrowScore {
    fn score(&self) -> i32;
}

impl ThrowScore for YourThrow {
    fn score(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}

trait DetermineThrow {
    fn determine_throw(&self) -> YourThrow;
}

impl DetermineThrow for RockPaperScissorsRoundPart2 {
    fn determine_throw(&self) -> YourThrow {
        match self.opponent_throw {
            OpponentThrow::Rock => {
                match self.round_result {
                    RoundResult::Lose => Scissors,
                    RoundResult::Draw => Rock,
                    RoundResult::Win => Paper
                }
            }
            OpponentThrow::Paper => {
                match self.round_result {
                    RoundResult::Lose => Rock,
                    RoundResult::Draw => Paper,
                    RoundResult::Win => Scissors
                }
            }
            OpponentThrow::Scissors => {
                match self.round_result {
                    RoundResult::Lose => Paper,
                    RoundResult::Draw => Scissors,
                    RoundResult::Win => Rock
                }
            }
        }
    }
}

trait RoundGetResult {
    fn get_result(&self) -> RoundResult;
}

impl RoundGetResult for RockPaperScissorsRoundPart1 {
    fn get_result(&self) -> RoundResult {
        match self.opponent_throw {
            OpponentThrow::Rock => {
                match self.your_throw {
                    Rock => RoundResult::Draw,
                    Paper => RoundResult::Win,
                    Scissors => RoundResult::Lose
                }
            }
            OpponentThrow::Paper => {
                match self.your_throw {
                    Rock => RoundResult::Lose,
                    Paper => RoundResult::Draw,
                    Scissors => RoundResult::Win
                }
            }
            OpponentThrow::Scissors => {
                match self.your_throw {
                    Rock => RoundResult::Win,
                    Paper => RoundResult::Lose,
                    Scissors => RoundResult::Draw
                }
            }
        }
    }
}

pub fn part1_again(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut total_score: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();
        let throws: Vec<&str> = line.split(' ').collect();

        let round = RockPaperScissorsRoundPart1 {
            opponent_throw: throws[0].parse().unwrap(),
            your_throw: throws[1].parse().unwrap()
        };

        total_score += round.get_result().score() + round.your_throw.score();
    }

    return total_score;
}

pub fn part2_again(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let reader = BufReader::new(&file);
    let lines = reader.lines();

    let mut total_score: i32 = 0;

    for (_index, line) in lines.enumerate() {
        let line = line.unwrap();
        let throws: Vec<&str> = line.split(' ').collect();

        let round = RockPaperScissorsRoundPart2 {
            opponent_throw: throws[0].parse().unwrap(),
            round_result: throws[1].parse().unwrap()
        };

        total_score += round.round_result.score() + round.determine_throw().score();
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
    fn test_part1_again() {
        assert_eq!(part1_again("resources/2022/day2-example.txt"), 15);
        assert_eq!(part1_again("resources/2022/day2.txt"), 14297);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("resources/2022/day2-example.txt"), 12);
        assert_eq!(part2("resources/2022/day2.txt"), 10498);
    }

    #[test]
    fn test_part2_again() {
        assert_eq!(part2_again("resources/2022/day2-example.txt"), 12);
        assert_eq!(part2_again("resources/2022/day2.txt"), 10498);
    }
}