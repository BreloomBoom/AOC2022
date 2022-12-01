use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::max;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("File could not open"));
    let mut current_calories = 0;
    let mut max_calories = 0;

    for line in reader.lines() {
        if line.as_ref().expect("Line could not be read").eq("") {
            max_calories = max(max_calories, current_calories);
            current_calories = 0;
        } else {
            current_calories += line.expect("Line could not be read").parse::<i32>().unwrap();
        }
    }

    max_calories = max(max_calories, current_calories);
    println!("The max calories for an elf was {} calories", max_calories);
}