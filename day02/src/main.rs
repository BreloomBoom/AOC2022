use std::io::BufReader;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = BufReader::new(File::open("input.txt").expect("File could not open"));
    let mut line = String::new();
    let _len = input.read_to_string(&mut line);
    
    let individual_scores = line
        .split('\n')
        .into_iter()
        .map(|rpsmatch| {
            let o = rpsmatch.split(' ').nth(0).unwrap().chars().next().unwrap() as u32;
            let u = rpsmatch.split(' ').nth(1).unwrap().chars().next().unwrap() as u32;
            (u - 'W' as u32 + (u - o + 2).rem_euclid(3) * 3,
            (u - 'X' as u32) * 3 + (o - 'A' as u32 + u - 'X' as u32 + 2).rem_euclid(3) + 1)
        });
    
    let scores = individual_scores.fold([0,0], |a, b| [a[0] + b.0, a[1] + b.1]);
    println!("The total score for part 1 was {}", scores[0]);
    println!("The total score for part 2 was {}", scores[1]);
}