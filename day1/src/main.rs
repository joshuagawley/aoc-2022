use std::collections::BinaryHeap;
use anyhow::Result;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Flatten;

type FlattenedLines = Flatten<Lines<BufReader<File>>>;

fn open_file(input: &str) -> Result<FlattenedLines> {
    let file = File::open(input)?;
    Ok(BufReader::new(file).lines().flatten())
}

fn part1(lines: &mut FlattenedLines) -> Result<String> {
    let mut max = i32::MIN;
    let mut num = 0;

    for line in lines {
        if line.is_empty() {
            max = max.max(num);
            num = 0;
        } else {
            num += line.parse::<i32>()?;
        }
    }

    Ok(max.to_string())
}

fn part2(lines: &mut FlattenedLines) -> Result<String> {
    let mut heap = BinaryHeap::new();
    let mut num = 0;

    for line in lines {
        if line.is_empty() {
            heap.push(num);
            num = 0;
        } else {
            num += line.parse::<i32>()?;
        }
    }

    let mut result = 0;
    for _ in 0..3 {
        if let Some(x) = heap.pop() {
            result += x;
        }
    }

    Ok(result.to_string())
}

fn main() -> Result<()> {
    let part1_result = part1(&mut open_file("input")?)?;
    let part2_result = part2(&mut open_file("input")?)?;

    println!("Part 1 answer: {}", part1_result);
    println!("Part 2 answer: {}", part2_result);

    Ok(())
}
