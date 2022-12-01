use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::max;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("File could not open"));
    let mut current = 0;
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;

    for line in reader.lines() {
        if line.as_ref().expect("Line could not be read").eq("") {
            if current > first {
                third = second;
                second = first;
                first = current;
            } else if current > second {
                third = second;
                second = current;
            } else if current > third {
                third = current;
            }
            current = 0;
        } else {
            current += line.expect("Line could not be read").parse::<i32>().unwrap();
        }
    }

    let total = max(first + second + third, first + second + current);
    let max = max(first, current);
    println!("The max calories for an elf was {} calories", max);
    println!("The max top 3 calories for the elves are {} calories", total);
}