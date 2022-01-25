use std::collections::HashSet;
use std::convert::TryInto;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<NoteEntry> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> NoteEntry {
    let (patterns_str, output_str) = line.split_once('|').unwrap();

    let signal_patterns = to_arr(
        patterns_str
            .trim()
            .splitn(10, ' ')
            .map(SignalPattern::from)
            .collect(),
    );
    let output_values = to_arr(
        output_str
            .trim()
            .splitn(4, ' ')
            .map(SignalPattern::from)
            .collect(),
    );

    NoteEntry {
        signal_patterns,
        output_values,
    }
}

fn to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub struct NoteEntry {
    signal_patterns: [SignalPattern; 10],
    output_values: [SignalPattern; 4],
}

#[derive(Debug)]
pub struct SignalPattern {
    set: HashSet<char>,
    len: u8,
}

struct DigitPattern {
    set: HashSet<char>,
    len: u8,
    digit: Digit,
}

enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

impl Into<u32> for Digit {
    fn into(self) -> u32 {
        match self {
            Digit::One => { 1 }
            Digit::Two => { 2 }
            Digit::Three => { 3 }
            Digit::Four => { 4 }
            Digit::Five => { 5 }
            Digit::Six => { 6 }
            Digit::Seven => { 7 }
            Digit::Eight => { 8 }
            Digit::Nine => { 9 }
            Digit::Zero => { 0 }
        }
    }
}

impl From<&str> for SignalPattern {
    fn from(str: &str) -> Self {
        let set: HashSet<char> = HashSet::from_iter(str.trim().chars());

        let len = str.trim().len() as u8;
        SignalPattern { set, len }
    }
}

#[aoc(day8, part1)]
pub fn run(input: &[NoteEntry]) -> usize {
    input
        .iter()
        .flat_map(|n| &n.output_values)
        .filter(|&pat| is_easy(pat.len))
        .count()
}

fn is_easy(len: u8) -> bool {
    matches!(len, 2 | 3 | 4 | 7)
}

// 1 -> 7 -> 3 -> 9 -> 8
// 1 -> 4 -> 9 -> 8
// 2 5 6 0

#[aoc(day8, part2)]
pub fn run_p2(input: &[NoteEntry]) -> usize {
    input.iter().map(determine_digits).map(|(entry, digits)| output_value(entry.output_values, digits))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day8.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 445);
    }

    // #[test]
    // fn input_known_answer_p2() {
    //     let result = run_p2(&input_generator(INPUT));
    //
    //     assert_eq!(result, 0);
    // }

    const EXAMPLE: &str = include_str!("../input/2021/day8_example.txt");

    #[test]
    fn example() {
        let actual = run(&input_generator(EXAMPLE));

        assert_eq!(actual, 26);
    }
}
