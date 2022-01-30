use std::collections::HashMap;
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
    string_rep: String,
    len: u8,
}

struct DigitPattern<'a> {
    string_rep: &'a str,
    digit: Digit,
}

#[derive(Copy, Clone)]
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
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
            Digit::Zero => 0,
        }
    }
}

impl From<&str> for SignalPattern {
    fn from(str: &str) -> Self {
        let mut sorted_chars = str.trim().chars().collect::<Vec<_>>();
        sorted_chars.sort_unstable();

        let string_rep: String = sorted_chars.into_iter().collect();

        let len = str.trim().len() as u8;
        SignalPattern { string_rep, len }
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

#[aoc(day8, part2)]
pub fn run_p2(input: &[NoteEntry]) -> usize {
    let entries = input.into_iter().map(determine_digits);
    let sum_u32: u32 = entries
        .map(|(entry, digit_mapping)| output_value(&entry.output_values, &digit_mapping))
        .sum();
    sum_u32 as usize
}

fn determine_digits<'a>(entry: &'a NoteEntry) -> (&'a NoteEntry, HashMap<&'a str, DigitPattern>) {
    struct KnownDigits<'a> {
        map: HashMap<&'a str, DigitPattern<'a>>,
    }
    impl<'a, 'b: 'a> KnownDigits<'a> {
        fn new() -> KnownDigits<'a> {
            KnownDigits { map: HashMap::new() }
        }

        fn add_known(&'a mut self, digit: Digit, sp: &'b str) {
            self.map.insert(
                sp,
                DigitPattern {
                    string_rep: sp,
                    digit,
                },
            );
        }
    }
    let mut known_digits: KnownDigits = KnownDigits::new();

    // do the magic
    // 1 -> 7 -> 3 -> 9 -> 8
    // 1 -> 4 -> 9 -> 8
    // 2 5 6 0
    let mut patterns: Vec<_> = entry.signal_patterns.iter().collect();
    patterns.sort_unstable_by_key(|p| p.len);
    // length 7
    let eight = patterns.remove(9);
    known_digits.add_known(Digit::Eight, &eight.string_rep);
    // length 2
    let one = patterns.remove(0);
    known_digits.add_known(Digit::One, &one.string_rep);
    // length 3
    let seven = patterns.remove(0);
    known_digits.add_known(Digit::Seven, &seven.string_rep);
    // length 4
    let four = patterns.remove(0);
    known_digits.add_known(Digit::Four, &four.string_rep);

    // left:
    // 2 (len 5);    6 (len 6)
    // 3 (len 5);    9 (len 6)
    // 5 (len 5);    0 (len 6)
    let mut len = patterns.chunks(3);
    let mut len5: Vec<&&SignalPattern> = len.next().unwrap().into_iter().collect();
    let mut len6: Vec<&&SignalPattern> = len.next().unwrap().into_iter().collect();

    let three_idx = len5
        .iter()
        .enumerate()
        .find(|(_, &sp)| sp.contains(&one))
        .expect("should exist (3)")
        .0;
    let three = len5.remove(three_idx);
    known_digits.add_known(Digit::Three, &three.string_rep);

    let nine_idx = len6
        .iter()
        .enumerate()
        .find(|(_, &sp)| sp.contains(&four))
        .expect("should exist (9)")
        .0;
    let nine = len6.remove(nine_idx);
    known_digits.add_known(Digit::Nine, &nine.string_rep);

    let zero_idx = len6
        .iter()
        .enumerate()
        .find(|(_, &sp)| sp.contains(&one))
        .expect("should exist (0)")
        .0;
    let zero = len6.remove(zero_idx);
    known_digits.add_known(Digit::Zero, &zero.string_rep);

    let six = len6.remove(0); // last one left
    known_digits.add_known(Digit::Six, &six.string_rep);

    let five_idx = len5
        .iter()
        .enumerate()
        .find(|(_, &sp)| six.contains(sp))
        .expect("should exist (5)")
        .0;
    let five = len5.remove(five_idx);
    known_digits.add_known(Digit::Five, &five.string_rep);

    let two = len5.remove(0); // last one left
    known_digits.add_known(Digit::Two, &two.string_rep);

    (entry, known_digits.map)
}

impl SignalPattern {
    fn contains(&self, other: &Self) -> bool {
        other
            .string_rep
            .chars()
            .all(|c| self.string_rep.contains(c))
    }
}

fn output_value(sp: &[SignalPattern; 4], digits_by_string: &HashMap<&str, DigitPattern>) -> u32 {
    let multipliers = [1000, 100, 10, 1];
    let digits: Vec<u32> = sp
        .iter()
        .map(|pattern| {
            digits_by_string
                .get(&*pattern.string_rep)
                .unwrap_or_else(|| {
                    panic!("digit pattern '{}' should be mapped", pattern.string_rep)
                })
                .digit
                .into()
        })
        .collect();
    digits
        .iter()
        .enumerate()
        .map::<u32, _>(|(pos, digit)| digit * multipliers[pos])
        .sum()
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
