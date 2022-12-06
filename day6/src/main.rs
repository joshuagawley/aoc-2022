use std::collections::HashSet;
use std::fs;

fn common<const N: usize>(input: &str) -> usize {
    for (i, _) in input.as_bytes().iter().enumerate() {
        let slice = &input[i..(i + N)];
        let slice_set = slice.as_bytes().iter().collect::<HashSet<_>>();
        if slice.len() == slice_set.len() {
            return i + N;
        }
    }

    input.len() - 1
}

fn part1(input: &str) -> usize {
    common::<4>(input)
}

fn part2(input: &str) -> usize {
    common::<14>(input)
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;

    let part1_result = part1(&input);
    let part2_result = part2(&input);

    println!("Part 1 result: {part1_result}");
    println!("Part 2 result: {part2_result}");

    Ok(())
}
