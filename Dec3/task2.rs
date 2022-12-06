use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let input = File::open("input")
        .expect("Unable to open input file");

    let mut priority_sum = 0;
    let mut lines = BufReader::new(input).lines();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {

        let rucksack_cmp1_str = line1.unwrap().to_owned();
        let rucksack_cmp2_str = line2.unwrap().to_owned();
        let rucksack_cmp3_str = line3.unwrap().to_owned();

        let mut common_item: char = '@';
        for rucksack1_item in rucksack_cmp1_str.chars() {
            for rucksack2_item in rucksack_cmp2_str.chars() {
                for rucksack3_item in rucksack_cmp3_str.chars() {
                    if rucksack1_item == rucksack2_item &&
                        rucksack2_item == rucksack3_item &&
                        rucksack3_item == rucksack1_item {
                        common_item = rucksack1_item;
                    }

                }
            }
        }
        if common_item == '@' {
            panic!("No common item found");
        }

        let priority: u32 = if common_item >= 'a' && common_item <= 'z' {
            (common_item as u32 - 'a' as u32) + 1
        }
        else if common_item >= 'A' && common_item <= 'Z' {
            (common_item as u32 - 'A' as u32) + 27
        }
        else {
            panic!("Priority convert error");
        };
        priority_sum += priority;
    }

    println!("Priority Sum: {}", priority_sum);
}

