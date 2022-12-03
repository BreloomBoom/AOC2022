#![feature(iter_array_chunks)]
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let priorities1 = input
        .split('\n')
        .map(|rucksack| {
            let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);
            priority(compartment1.chars().filter(|c| compartment2.contains(*c)).next().unwrap() as u32)
        })
        .fold(0, |a, b| a + b);
    
    let priorities2 = input
        .split('\n')
        .array_chunks()
        .map(|[a, b, c]| priority(a.chars().filter(|ch| b.contains(*ch) && c.contains(*ch)).next().unwrap() as u32))
        .fold(0, |a, b| a + b);

    println!("The sum of priorities for part 1 is {}.", priorities1);
    println!("The sum of priorities for part 2 is {}.", priorities2);
}

fn priority(letter: u32) -> u32 {
    if letter >= 'a' as u32 {
        letter - 'a' as u32 + 1
    } else {
        letter - 'A' as u32 + 27
    }
}