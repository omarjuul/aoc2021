use core::panicking::panic;
use crate::day2::MoveCommand::{Down, Forward, Up};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<MoveCommand> {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|l| {
            let amount = l.1.parse::<u32>().unwrap();
            match l.0 {
                "forward" => Forward(amount),
                "down" => Down(amount),
                "up" => Up(amount),
                _ => panic!("unexpected move command {}", l.0)
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn run(input: &[MoveCommand]) -> usize {
    // input.into_iter()
    let increases = input.windows(2).map(|w| w[0] < w[1]).filter(|b| *b).count();

    increases
}

/*#[aoc(day2, part2)]
pub fn run_windowed_average(input: &[u32]) {}*/

enum MoveCommand {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day1.txt");
}
