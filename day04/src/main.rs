use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The number of self-contained pairs are {}", sol(&input)[0]);
    println!("The number of self-contained pairs are {}", sol(&input)[1]);
}

fn sol(input: &str) -> [u32; 2] {
    input
        .split('\n')
        .map(|pair| {
            let (range1, range2) = pair.split_at(pair.find(',').unwrap());
            let (l1, u1) = range1.split_at(range1.find('-').unwrap());
            let (l2, u2) = range2[1..].split_at(range2.find('-').unwrap() - 1);
            [if to_i32(u1) - to_i32(l1) > to_i32(u2) - to_i32(l2) {
                (to_i32(l2) >= to_i32(l1) && to_i32(u2) <= to_i32(u1)) as u32
            } else {
                (to_i32(l1) >= to_i32(l2) &&  to_i32(u1) <= to_i32(u2)) as u32
            }, 
            ((to_i32(l2) <= to_i32(u1) && to_i32(u1) <= to_i32(u2)) || 
            (to_i32(l1) <= to_i32(u2) && to_i32(u2) <= to_i32(u1)))
            as u32]
        })
        .fold([0, 0], |a, b| [a[0] + b[0], a[1] + b[1]])
}

fn to_i32(no: &str) -> i32 {
    if no.chars().next().unwrap() == '-' {
        no[1..].parse::<i32>().unwrap()
    } else {
        no.parse::<i32>().unwrap()
    }
}
