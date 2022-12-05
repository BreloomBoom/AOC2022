use std::fs;
use std::collections::LinkedList;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
 
    println!("The top crates are {}", sol(input.clone()));
}

fn sol(input: String) -> String {
    let mut crates_commands = input.split("\n\n");
    let unparsed_crates = crates_commands.next().unwrap();
    let mut crates = parse_crates(unparsed_crates.clone());
    let mut crates2 = parse_crates(unparsed_crates);

    for command in crates_commands.next().unwrap().split('\n') {
        let mut parts = command.split(" from ");
        let crate_no = (parts.next().unwrap())[5..].parse::<i32>().unwrap();

        let from_to = parts.next().unwrap();
        let from = from_to[0..1].parse::<usize>().unwrap();
        let to = from_to[5..6].parse::<usize>().unwrap();

        let mut stack = LinkedList::new();
        for _ in 0..crate_no {
            let single_crate = (crates[from - 1]).pop_back().unwrap();
            stack.push_back(single_crate.clone());
            (crates[to - 1]).push_back(single_crate);
        }
        for _ in 0..crate_no {
            (crates2[to - 1]).push_back(stack.pop_back().unwrap());
        }
    }

    let part1 = crates
        .into_iter()
        .map(|crate_list| *(crate_list.back().unwrap()))
        .collect::<String>();
    let part2 = crates2
        .into_iter()
        .map(|crate_list| *(crate_list.back().unwrap()))
        .collect::<String>();
    format!("{} and {}", part1, part2)
}

fn parse_crates(unparsed_crates: &str) -> Vec<LinkedList<char>> {
    let mut crates: Vec<LinkedList<char>> = vec![LinkedList::new(); 9];
    for row in unparsed_crates.split('\n') {
        for i in 0..9 {
            let character = row[(1 + 4 * i)..(2 + 4 * i)].chars().next().unwrap();
            if character != ' ' {
                (crates[i]).push_front(character);
            }
        }
    }
    crates
}
