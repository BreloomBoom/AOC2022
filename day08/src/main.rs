use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The number of visible trees is {}", part1(input.clone()));
    println!("The best score is {}", part2(input));
}

fn part1(input: String) -> usize {
    let trees = parse_trees(input);
    let width = trees[0].len();
    let height = trees.len();
    let mut visible: usize = 0;

    for i in 0..height - 1 {
        for j in 0..width - 1 {
            visible += is_visible(trees.clone(), i, j);
        }
    }
    visible += width + height - 1;

    visible
}

fn parse_trees(input: String) -> Vec<Vec<u32>> {
    let mut trees: Vec<Vec<u32>> = vec![];
    for row in input.split('\n') {
        trees.push(row.chars().map(|tree| tree.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }
    trees
}

fn is_visible(trees: Vec<Vec<u32>>, row: usize, col: usize) -> usize {
    if trees[0..row].iter().all(|height| height[col] < trees[row][col]) {
        return 1;
    }

    if trees[row + 1..].iter().all(|height| height[col] < trees[row][col]) {
        return 1;
    }

    if trees[row][0..col].iter().all(|&height| height < trees[row][col]) {
        return 1;
    }

    if trees[row][col + 1..].iter().all(|&height| height < trees[row][col]) {
        return 1;
    }

    0
}

fn part2(input: String) -> usize {
    let trees = parse_trees(input);
    let width = trees[0].len();
    let height = trees.len();
    let mut score: usize = 0;

    for i in 0..height - 2 {
        for j in 0..width - 2 {
            let curr = calc_score(trees.clone(), i + 1, j + 1, width, height);
            if curr > score {
                score = curr;
            }
        }
    }
    score
}

fn calc_score(trees: Vec<Vec<u32>>, row: usize, col: usize, width: usize, height: usize) -> usize {
    let mut up: usize = 1;
    let mut i = row - 1;
    while i > 0 && trees[i][col] < trees[row][col] {
        i -= 1;
        up += 1;
    }
    

    let mut down: usize = 1;
    i = row + 1;
    while i < height - 1 && trees[i][col] < trees[row][col] {
        i += 1;
        down += 1;
    }

    let mut left: usize = 1;
    i = col - 1;
    while i > 0 && trees[row][i] < trees[row][col] {
        i -= 1;
        left += 1;
    }
    
    let mut right: usize = 1;
    i = col + 1;
    while i < width - 1 && trees[row][i] < trees[row][col] {
        i += 1;
        right += 1;
    }
    
    up * down * left * right
}