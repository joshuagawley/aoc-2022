use common::*;
use std::collections::HashSet;
use std::hash::Hash;

fn accumulate_result(mut acc: i32, c: char) -> i32 {
    if c.is_ascii_uppercase() {
        acc += ((c as u8) - 38) as i32;
    } else {
        acc += ((c as u8) - b'a' + 1) as i32;
    }
    acc
}

fn inplace_intersection<T>(a: &mut HashSet<T>, b: &mut HashSet<T>) -> HashSet<T>
where
    T: Hash,
    T: Eq,
{
    let x = a
        .drain()
        .map(|v| {
            let intersects = b.contains(&v);
            (v, intersects)
        })
        .collect::<HashSet<(T, bool)>>();

    let mut result = HashSet::new();

    for (v, is_intersect) in x {
        if is_intersect {
            result.insert(v);
        } else {
            a.insert(v);
        }
    }

    b.retain(|v| !result.contains(&v));

    result
}

fn part1(input: &mut FlattenedLines) -> i32 {
    let mut result = 0;

    for line in input {
        let first_comparment = line[0..(line.len() / 2)].chars().collect::<HashSet<char>>();
        let second_compartment = line[((line.len() / 2)..line.len())]
            .chars()
            .collect::<HashSet<char>>();

        let common_chars = first_comparment
            .intersection(&second_compartment)
            .collect::<Vec<&char>>();

        result += common_chars.iter().fold(0, |mut acc, &c| {
            if c.is_ascii_uppercase() {
                acc += ((*c as u8) - 38) as i32;
            } else {
                acc += ((*c as u8) - b'a' + 1) as i32;
            }
            acc
        });
    }

    result
}

fn part2(input: &mut FlattenedLines) -> i32 {
    let mut result = 0;
    let mut first_group = HashSet::new();
    let mut second_group = HashSet::new();
    for (index, line) in input.enumerate() {
        match index % 6 {
            0 => {
                first_group = line.chars().collect::<HashSet<char>>();
            }
            1 => {
                first_group = inplace_intersection(
                    &mut first_group,
                    &mut line.chars().collect::<HashSet<char>>(),
                )
            }
            2 => {
                first_group = inplace_intersection(
                    &mut first_group,
                    &mut line.chars().collect::<HashSet<char>>(),
                );

                result += first_group.iter().fold(0, |mut acc, &c| {
                    if c.is_ascii_uppercase() {
                        acc += ((c as u8) - 38) as i32;
                    } else {
                        acc += ((c as u8) - b'a' + 1) as i32;
                    }
                    acc
                });
            }
            3 => {
                second_group = line.chars().collect::<HashSet<char>>();
            }
            4 => {
                second_group = inplace_intersection(
                    &mut second_group,
                    &mut line.chars().collect::<HashSet<char>>(),
                )
            }
            5 => {
                second_group = inplace_intersection(
                    &mut second_group,
                    &mut line.chars().collect::<HashSet<char>>(),
                );

                result += second_group.iter().fold(0, |mut acc, &c| {
                    if c.is_ascii_uppercase() {
                        acc += ((c as u8) - 38) as i32;
                    } else {
                        acc += ((c as u8) - b'a' + 1) as i32;
                    }
                    acc
                });
            }
            _ => unimplemented!(),
        }
    }
    result
}

fn main() -> anyhow::Result<()> {
    let part1_result = part1(&mut open_file("input")?);
    let part2_result = part2(&mut open_file("input")?);

    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);

    for x in 1_usize..10 {
        println!("{}", x % 6)
    }

    Ok(())
}
