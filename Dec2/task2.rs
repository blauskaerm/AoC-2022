use std::fs::File;
use std::io::{BufReader, BufRead, Error};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Rps{
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RpsOutcome{
    Lose = 1,
    Draw = 2,
    Win = 3,
}

fn main() -> Result<(), Error> {

    let input = File::open("input")
        .expect("Unable to open input file");
    let buffered = BufReader::new(input);

    let mut my_score = 0;
    for line in buffered.lines() {

        let line = line.unwrap();
        let line_split_vec: Vec<&str> = line.split(" ").collect();

        if line_split_vec.iter().count() == 2 {
            let opponent_str = line_split_vec[0];
            let outcome_str = line_split_vec[1];

            let opponent = match opponent_str {
                "A" => Rps::Rock,
                "B" => Rps::Paper,
                "C" => Rps::Scissors,
                _ => panic!("Opponent parse error"),
            };
            let outcome = match outcome_str {
                "X" => RpsOutcome::Lose,
                "Y" => RpsOutcome::Draw,
                "Z" => RpsOutcome::Win,
                _ => panic!("Player parse error"),
            };

            let score_comp_1: u32;
            let score_comp_2: u32;
            match outcome {
                RpsOutcome::Draw => {
                    score_comp_1 = opponent as u32;
                    score_comp_2 = 3;
                },
                RpsOutcome::Lose => {
                    score_comp_1 = match opponent {
                        Rps::Rock => Rps::Scissors as u32,
                        Rps::Paper => Rps::Rock as u32,
                        Rps::Scissors => Rps::Paper as u32,
                    };
                    score_comp_2 = 0;
                },
                RpsOutcome::Win => {
                    score_comp_1 = match opponent {
                        Rps::Rock => Rps::Paper as u32,
                        Rps::Paper => Rps::Scissors as u32,
                        Rps::Scissors => Rps::Rock as u32,
                    };
                    score_comp_2 = 6;
                },
            };

            my_score += score_comp_1 + score_comp_2;
        }
        else {
            panic!("Format error");
        }
    }

    println!("Score: {}", my_score);

    Ok(())
}

