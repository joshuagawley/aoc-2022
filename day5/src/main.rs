use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;

fn parse_stack(input: &str) -> Vec<VecDeque<char>> {
    let mut result = Vec::new();

    let mut column_indices = Vec::new();

    for (index, _) in input
        .lines()
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|x| x.1.is_ascii_digit())
    {
        result.push(VecDeque::new());
        column_indices.push(index);
    }

    for line in input.lines().filter(|&x| !x.is_empty()) {
        for (index, column) in line.chars().enumerate().filter(|val| {
            column_indices.contains(&val.0)
                && !(val.1.is_ascii_whitespace() || val.1.is_ascii_digit())
        }) {
            for (idx, _) in column_indices.iter().enumerate().filter(|x| *x.1 == index) {
                result[idx].push_back(column);
            }
        }
    }

    result
}

fn parse_instructions(input: &str) -> Vec<(usize, usize, usize)> {
    let mut result = Vec::new();

    input.lines().fold(&mut result, |acc, line| {
        acc.push(
            line.split_ascii_whitespace()
                .filter(|x| x.chars().all(|x| x.is_ascii_digit()))
                .map(|x| x.parse::<usize>().unwrap())
                .next_tuple::<(usize, usize, usize)>()
                .unwrap(),
        );
        acc
    });

    result
}

fn part1(input: &str) -> String {
    let (stacks, instructions) = input.split("\n\n").next_tuple().unwrap();
    let mut stacks = parse_stack(stacks);
    let instructions = parse_instructions(instructions);

    for instruction in instructions {
        for _ in 0..instruction.0 {
            if let Some(x) = stacks[instruction.1 - 1].pop_front() {
                stacks[instruction.2 - 1].push_front(x);
            }
        }
    }

    let mut result = String::new();

    for stack in stacks {
        if let Some(x) = stack.front() {
            if x.is_alphabetic() {
                result.push(*x);
            }
        }
    }

    result
}

fn part2(input: &str) -> String {
    let (stacks, instructions) = input.split("\n\n").next_tuple().unwrap();
    let mut stacks = parse_stack(stacks);
    let instructions = parse_instructions(instructions);

    for instruction in instructions {
        let mut temp = VecDeque::new();
        for _ in 0..instruction.0 {
            if let Some(x) = stacks[instruction.1 - 1].pop_front() {
                temp.push_front(x);
            }
        }
        for val in temp {
            stacks[instruction.2 - 1].push_front(val);
        }
    }

    let mut result = String::new();

    for stack in stacks {
        if let Some(x) = stack.front() {
            if x.is_alphabetic() {
                result.push(*x);
            }
        }
    }

    result
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;

    let part1_result = part1(&input);
    let part2_result = part2(&input);

    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let input = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(part1(input), "CMZ");
        assert_eq!(part2(input), "MCD");
    }
}
