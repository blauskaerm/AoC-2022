use std::fs::File;
use std::io::{BufReader, BufRead, Error};

#[derive(Debug, PartialEq, Eq)]
enum Rps{
    Rock = 1,
    Paper = 2,
    Scissors = 3,
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
            let player_str = line_split_vec[1];
            let round_score;

            let opponent = match opponent_str {
                "A" => Rps::Rock,
                "B" => Rps::Paper,
                "C" => Rps::Scissors,
                _ => panic!("Opponent parse error"),
            };
            let player = match player_str {
                "X" => Rps::Rock,
                "Y" => Rps::Paper,
                "Z" => Rps::Scissors,
                _ => panic!("Player parse error"),
            };

            round_score = (player as u32) +

            /* Win */
            if player == Rps::Paper && opponent == Rps::Rock {6}
            else if player == Rps::Rock && opponent == Rps::Scissors {6}
            else if player == Rps::Scissors && opponent == Rps::Paper {6}

            /* Draw */
            else if player == opponent {3}

            /* Loss */
            else {0};

            my_score += round_score;
        }
        else {
            panic!("Format error");
        }
    }

    println!("Score: {}", my_score);

    Ok(())
}

