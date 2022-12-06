use std::fs::File;
use std::io::{BufReader, BufRead};

fn get_region(region_str: &str) -> (u32, u32) {

    let region_split_vec: Vec<&str> = region_str.split("-").collect();
    if region_split_vec.iter().count() != 2 {
        panic!("Split region error");
    }
    let low = region_split_vec[0].parse::<u32>();
    if low.is_err() {
        panic!("Low region parse error");
    }
    let low = low.unwrap();

    let high = region_split_vec[1].parse::<u32>();
    if high.is_err() {
        panic!("High region parse error");
    }
    let high = high.unwrap();

    (low, high)
}

fn print_region(low: u32, high: u32) {
    for _i in 1..low {
        print!(".");
    }
    for i in low..=high {
        print!("{}", i);
    }
    for _i in high..9 {
        print!(".");
    }
    println!();
}

fn main() {

    let input = File::open("input")
        .expect("Unable to open input file");

    let mut overlap_count = 0;
    for line in BufReader::new(input).lines() {

        let line = line.unwrap();
        let line_split_vec: Vec<&str> = line.split(",").collect();
        if line_split_vec.iter().count() != 2 {
            panic!("Split format error");
        }

        let region_1_str = line_split_vec[0];
        let region_2_str = line_split_vec[1];

        let (region_1_low, region_1_high) = get_region(&region_1_str);
        let (region_2_low, region_2_high) = get_region(&region_2_str);

        if (region_1_low <= region_2_low && region_1_high >= region_2_high)
            || region_2_low <= region_1_low && region_2_high >= region_1_high{
            overlap_count += 1;
        }
    }

    println!("Overlaps: {}", overlap_count);
}
