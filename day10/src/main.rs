use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The sum of these signal strengths is {}", part1(input.clone()));
    println!("The CRT Image is {}", part2(input));
}

fn part1(input: String) -> i32 {
    let mut cycle: i32 = 1;
    let mut register: i32 = 1;
    let mut to_add: i32 = 0;
    let signals = [20, 60, 100, 140, 180, 220];
    let mut sum: i32 = 0;

    let mut commands = input.split('\n');
    let mut command = commands.next();
    while command.is_some() {
        if signals.contains(&cycle) {
            sum += cycle * register;
        }

        let comm = command.unwrap();
        cycle += 1;
        if comm == "noop" {
            command = commands.next();
        } else if to_add != 0 {
            register += to_add;
            to_add = 0;
            command = commands.next();
        } else {
            to_add = comm.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
    }

    sum
}

fn part2(input: String) -> String {
    let mut output = String::from("\n");
    let mut cycle: i32 = 1;
    let mut register: i32 = 1;
    let mut to_add: i32 = 0;
    let cycles = [41, 81, 121, 161, 201];

    let mut commands = input.split('\n');
    let mut command = commands.next();
    while command.is_some() {
        if cycles.contains(&cycle) {
            output.push('\n');
        }

        if (cycle % 40 - register - 1).abs() <= 1 {
            output.push('#');
        } else {
            output.push('.');
        }

        let comm = command.unwrap();
        cycle += 1;
        if comm == "noop" {
            command = commands.next();
        } else if to_add != 0 {
            register += to_add;
            to_add = 0;
            command = commands.next();
        } else {
            to_add = comm.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
    }

    output
}