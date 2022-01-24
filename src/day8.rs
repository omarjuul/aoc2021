use std::convert::TryInto;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<NoteEntry> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> NoteEntry {
    let (patterns_str, output_str) = line.split_once('|').unwrap();

    let signal_patterns = to_arr(patterns_str.splitn(10, ' ').map(|str| SignalPattern::from(str)).collect());
    let output_values = to_arr(output_str.splitn(4, ' ').map(|str| SignalPattern::from(str)).collect());
    NoteEntry { signal_patterns, output_values }
}

fn to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

struct NoteEntry {
    signal_patterns: [SignalPattern; 10],
    output_values: [SignalPattern; 4],
}

struct SignalPattern {
    code: u8,
    len: u8,
}

impl From<&str> for SignalPattern {
    fn from(str: &str) -> Self {
        let code = str.trim().chars().map(|c| match c {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            _ => panic!("unexpected character {}", c)
        }).sum();
        let len = str.trim().len() as u8;
        SignalPattern { code, len }
    }
}

#[aoc(day8, part1)]
pub fn run(input: &[NoteEntry]) -> usize {
    input.iter().flat_map(|n| n.output_values).filter(|pat| is_easy(pat.len)).count()
}

fn is_easy(len: u8) -> bool {
    match len {
        2 => true,
        3 => true,
        4 => true,
        8 => true,
        _ => false
    }
}

// #[aoc(day8, part2)]
// pub fn run_p2(input: &[u32]) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day8.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 0);
    }

    // #[test]
    // fn input_known_answer_p2() {
    //     let result = run_p2(&input_generator(INPUT));
    // 
    //     assert_eq!(result, 0);
    // }

    const EXAMPLE: [u32; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn example() {
        let actual = run(&EXAMPLE);

        assert_eq!(actual, 0);
    }
}