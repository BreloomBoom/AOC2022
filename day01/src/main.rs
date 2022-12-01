use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::max;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("File could not open"));
    let mut current = 0;
    let mut calories = vec![0];

    for line in reader.lines() {
        if line.as_ref().expect("Line could not be read").eq("") {
            if current > calories[0] {
                calories.insert(0, current);
            } else if current > calories[1] {
                calories.insert(1, current);
            } else if current > calories[2] {
                calories.insert(2, current);
            }
            current = 0;
        } else {
            current += line.expect("Line could not be read").parse::<i32>().unwrap();
        }
    }

    let total = max(calories[0] + calories[1] + calories[2], calories[0] + calories[1] + current);
    let max = max(calories[0], current);
    println!("The max calories for an elf was {} calories", max);
    println!("The max top 3 calories for the elves are {} calories", total);
}