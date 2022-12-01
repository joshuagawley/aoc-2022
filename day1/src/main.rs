use std::collections::BinaryHeap;
use anyhow::Result;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut heap = BinaryHeap::new();
    let mut num = 0;

    for line in reader.lines().flatten() {
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
    println!("{}", result);

    Ok(())
}
