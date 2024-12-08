use std::fs::File;
use std::io::{self, BufRead};

use day02::{check_safety, check_safety_dampened};

fn main() {
    let filename = "data/input.txt";
    let file = File::open(filename).expect("Failed to open the input.txt. file.");
    let reader = io::BufReader::new(file);

    let mut cnt_safe_lines = 0;
    let mut cnt_dampened_safe_lines = 0;
    let mut cnt_lines = 0;
    for line in reader.lines() {
        let mut data: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse number"))
            .collect();
        if check_safety(&data) {
            cnt_safe_lines += 1;
        }
        if check_safety_dampened(&mut data) {
            cnt_dampened_safe_lines += 1;
        }
        cnt_lines += 1;
    }
    println!("{} / {} reports are safe.", cnt_safe_lines, cnt_lines);
    println!(
        "{} / {} reports are safe when using the dampener.",
        cnt_dampened_safe_lines, cnt_lines
    );
}
