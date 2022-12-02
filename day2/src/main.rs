use anyhow::Result;
use common::*;

fn part1(input: &mut FlattenedLines) -> i32 {
    let mut score = 0;
    for line in input {
        let mut chars = line.chars();
        let elf_char = chars.next().unwrap();
        chars.next();
        let my_char = chars.next().unwrap();

        match my_char {
            'X' => {
                score += 1;
                match elf_char {
                    'A' => score += 3,
                    'C' => score += 6,
                    _ => {}
                }
            }
            'Y' => {
                score += 2;
                match elf_char {
                    'A' => score += 6,
                    'B' => score += 3,
                    _ => {}
                }
            }
            'Z' => {
                score += 3;
                match elf_char {
                    'C' => score += 3,
                    'B' => score += 6,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    score
}

fn part2(input: &mut FlattenedLines) -> i32 {
    let mut score = 0;

    for line in input {
        let mut chars = line.chars();
        let elf_char = chars.next().unwrap();
        chars.next();
        let my_char = chars.next().unwrap();

        match my_char {
            'X' => match elf_char {
                'A' => score += 3,
                'B' => score += 1,
                'C' => score += 2,
                _ => {}
            },
            'Y' => {
                score += 3;
                match elf_char {
                    'A' => score += 1,
                    'B' => score += 2,
                    'C' => score += 3,
                    _ => {}
                }
            }
            'Z' => {
                score += 6;
                match elf_char {
                    'A' => score += 2,
                    'B' => score += 3,
                    'C' => score += 1,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    score
}

fn main() -> Result<()> {
    let part1_result = part1(&mut open_file("input")?);
    let part2_result = part2(&mut open_file("input")?);

    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = "A Y\nB X\nC Z";
        let mut file = File::create("test")?;
        file.write_all(input.as_ref())?;
        assert_eq!(part1(&mut open_file("test")?), 15);
        Ok(())
    }
}
