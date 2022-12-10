use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The number of positions in part 1 is {}", part1(input.clone()));
    println!("The number of positions in part 2 is {}", part2(input));
}

fn part1(input: String) -> usize {
    let mut positions = vec![[0,0]];
    let mut head = [0,0];
    let mut tail = [0,0];
    for command in input.split('\n') {
        let args = command.split(' ').collect::<Vec<&str>>();
        let mut length = args[1].parse::<i32>().unwrap();
        if args[0] == "D" || args[0] == "L" {
            length = -length;
        }
        let increment = length / length.abs();
        for _ in 0..length.abs() {
            if args[0] == "U" || args[0] == "D" {
                head[1] += increment;
                if head[1] != tail[1] && head[1] - increment != tail[1] {
                    tail = [head[0], head[1] - increment];
                }
            } else {
                head[0] += increment;
                if head[0] != tail[0] && head[0] - increment != tail[0] {
                    tail = [head[0] - increment, head[1]];
                }
            }
            if !is_in(positions.clone(), tail) {
                positions.push(tail.clone());
            }
        }
    }

    positions.len()
}

fn is_in(positions: Vec<[i32; 2]>, tail: [i32; 2]) -> bool {
    for position in positions.into_iter() {
        if position[0] == tail[0] && position[1] == tail[1] {
            return true;
        }
    }
    false
}

fn part2(input: String) -> usize {
    let mut positions = vec![[0,0]];
    let mut string = [[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0]];
    for command in input.split('\n') {
        let args = command.split(' ').collect::<Vec<&str>>();
        let mut length = args[1].parse::<i32>().unwrap();
        if args[0] == "D" || args[0] == "L" {
            length = -length;
        }
        let increment = length / length.abs();
        for _ in 0..length.abs() {
            if args[0] == "U" || args[0] == "D" {
                string[0][1] += increment;
                for i in 0..9 {
                    let rowdiff = string[i][0] - string[i + 1][0];
                    let coldiff = string[i][1] - string[i + 1][1];

                    if rowdiff == 0 && coldiff.abs() > 1 {
                        string[i + 1][1] += coldiff.signum();
                    } else if coldiff == 0 && rowdiff.abs() > 1 {
                        string[i + 1][0] += rowdiff.signum();
                    } else if rowdiff.abs() > 1 || coldiff.abs() > 1 {
                        string[i + 1][0] += rowdiff.signum();
                        string[i + 1][1] += coldiff.signum();
                    }
                }
            } else {
                string[0][0] += increment;
                for i in 0..9 {
                    let rowdiff = string[i][0] - string[i + 1][0];
                    let coldiff = string[i][1] - string[i + 1][1];

                    if rowdiff == 0 && coldiff.abs() > 1 {
                        string[i + 1][1] += coldiff.signum();
                    } else if coldiff == 0 && rowdiff.abs() > 1 {
                        string[i + 1][0] += rowdiff.signum();
                    } else if rowdiff.abs() > 1 || coldiff.abs() > 1 {
                        string[i + 1][0] += rowdiff.signum();
                        string[i + 1][1] += coldiff.signum();
                    }
                }
            }
            if !is_in(positions.clone(), string[9]) {
                positions.push(string[9].clone());
            }
        }
    }

    positions.len()
}