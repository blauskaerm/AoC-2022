use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let input = File::open("input")
        .expect("Unable to open input file");
    let buffered = BufReader::new(input);

    let mut priority_sum = 0;
    for line in buffered.lines() {

        let rucksack_str = line.unwrap();
        let rucksack_str_len = rucksack_str.len();
        if rucksack_str_len % 2 != 0 {
            panic!("Rucksack format error");
        }

        let rucksack_cmp1_str = &rucksack_str[0..(rucksack_str_len) / 2].to_owned();
        let rucksack_cmp2_str = &rucksack_str[rucksack_str_len / 2..rucksack_str_len].to_owned();
        if rucksack_cmp1_str.len() != rucksack_cmp2_str.len() {
            panic!("Rucksack compartment size error");
        }

        let mut common_item: char = '@';
        for rucksack1_item in rucksack_cmp1_str.chars() {
            for rucksack2_item in rucksack_cmp2_str.chars() {
                if rucksack1_item == rucksack2_item {
                    common_item = rucksack1_item;
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

