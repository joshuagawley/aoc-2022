use common::*;
use itertools::Itertools;

fn get_ranges(line: String) -> Vec<(i32, i32)> {
    line.split(',')
        .map(|x| {
            x.split('-')
                .map(|x| x.parse::<i32>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn range_fully_contained(range: Vec<(i32, i32)>) -> bool {
    (range[0].0 <= range[1].0 && range[1].1 <= range[0].1)
        || (range[1].0 <= range[0].0 && range[0].1 <= range[1].1)
}

fn range_overlap(range: Vec<(i32, i32)>) -> bool {
    i32::min(range[0].1, range[1].1) >= i32::max(range[0].0, range[1].0)
}

fn common(input: &mut FlattenedLines, pred: &dyn Fn(Vec<(i32, i32)>) -> bool) -> i32 {
    input.fold(0, |x, str| if pred(get_ranges(str)) { x + 1 } else { x })
}

fn part1(input: &mut FlattenedLines) -> i32 {
    common(input, &range_fully_contained)
}

fn part2(input: &mut FlattenedLines) -> i32 {
    common(input, &range_overlap)
}

fn main() -> anyhow::Result<()> {
    let part1_result = part1(&mut open_file("input")?);
    let part2_result = part2(&mut open_file("input")?);

    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}
