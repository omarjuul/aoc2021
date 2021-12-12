#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn run(input: &[u32]) -> usize {
    let increases = input.windows(2).map(|w| w[0] < w[1]).filter(|b| *b).count();

    increases
}

#[aoc(day1, part2)]
pub fn run_windowed_average(input: &[u32]) -> usize {
    let windowed_averages = input
        .windows(3)
        .map(|w| w.into_iter().sum())
        .collect::<Vec<u32>>();

    run(&windowed_averages)
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
    fn input_known_answer_part2() {
        let result = run_windowed_average(&input_generator(INPUT));

        assert_eq!(result, 1429);
    }

    const EXAMPLE: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn example_increases_7_times() {
        let increases = run(&EXAMPLE);

        assert_eq!(increases, 7);
    }

    #[test]
    fn example_average_increases_5_times() {
        let increases = run_windowed_average(&EXAMPLE);

        assert_eq!(increases, 5);
    }
}
