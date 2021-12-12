#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn run(input: &[u32]) -> usize {
    let increases = input
        .windows(2)
        .map(|w| w[0] < w[1])
        .filter(|b| *b)
        .count();

    increases
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day1.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 1400);
    }

    #[test]
    fn example_increases_7_times() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let increases = run(&input);

        assert_eq!(increases, 7);
    }
}
