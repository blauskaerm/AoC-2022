use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {

    let input = File::open("input")
        .expect("Unable to open input file");
    let buffered = BufReader::new(input);

    let mut calories_max: u32 = 0;
    let mut elf_calories: u32 = 0;
    let mut elf_id = 0;
    for line in buffered.lines() {
        let calories_str = line.unwrap();

        if calories_str.len() > 0 {
            let calories = calories_str.parse::<u32>().unwrap();
            elf_calories += calories;
        }
        else {
            println!("Elf {} consumes {} Cal", elf_id, elf_calories);

            if elf_calories > calories_max {
                calories_max = elf_calories;
            }

            elf_calories = 0;
            elf_id += 1;
        }

        // println!("{}", calories_str);

    }

    println!("Max calories: {}", calories_max);

    Ok(())
}

