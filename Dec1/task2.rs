use std::fs::File;
use std::io::{BufReader, BufRead, Error};

macro_rules! calc_max_cal {
    ($cal_array:expr, $cal:expr) => {{
        if $cal > $cal_array[0] {
            $cal_array[2] = $cal_array[1];
            $cal_array[1] = $cal_array[0];
            $cal_array[0] = $cal;
        }
        else if $cal > $cal_array[1] {
            $cal_array[2] = $cal_array[1];
            $cal_array[1] = $cal;
        }
        else if $cal > $cal_array[2] {
            $cal_array[2] = $cal;
        }
    }};
}

fn main() -> Result<(), Error> {

    let input = File::open("input")
        .expect("Unable to open input file");
    let buffered = BufReader::new(input);

    let mut calories_max: [u32; 3] = [0,0,0];
    let mut elf_calories: u32 = 0;
    for line in buffered.lines() {
        let calories_str = line.unwrap();

        if calories_str.len() > 0 {
            let calories = calories_str.parse::<u32>().unwrap();
            elf_calories += calories;
        }
        else {
            calc_max_cal!(calories_max, elf_calories);
            elf_calories = 0;
        }
    }

    calc_max_cal!(calories_max, elf_calories);
    println!("Top calories: {}",
             calories_max[0] + calories_max[1] + calories_max[2]);

    Ok(())
}

