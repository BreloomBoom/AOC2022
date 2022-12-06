use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The first start-of-packet marker is detected at {}", part1(&input));
    println!("The first start-of-packet marker is detected at {}", part2(&input).unwrap());
}

fn part1(input: &str) -> usize {
    let len = input.len();
    (0..len - 3).filter(|i| unique(&input[*i..i + 4]).is_none()).next().unwrap() + 4
}


fn part2(input: &str) -> Option<usize> {
    let len = input.len();
    for i in 0..len - 3 {
        if unique(&input[i..i + 14]) == None {
            return Some(i + 14)
        }
    }
    None
}

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}