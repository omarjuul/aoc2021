#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u16>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn run(input: &[u16]) -> u32 {
    search_optimal_fuel_cost(input, |n| n)
}

#[aoc(day7, part2)]
pub fn run_p2(input: &[u16]) -> u32 {
    search_optimal_fuel_cost(input, |n| ((n + 1) * n) / 2)
}

fn search_optimal_fuel_cost(positions: &[u16], cost_fn: fn(u32) -> u32) -> u32 {
    let max_pos = positions.iter().max().unwrap();
    let min_pos = positions.iter().min().unwrap();
    let range = *min_pos..=*max_pos;
    let min_fuel_cost = range
        .map(|align_pos| calculate_fuel_cost(positions, align_pos, cost_fn))
        .min();
    min_fuel_cost.unwrap()
}

fn calculate_fuel_cost(positions: &[u16], align_at: u16, cost_fn: fn(u32) -> u32) -> u32 {
    positions
        .iter()
        .map(|&p| cost_fn((align_at as i32 - p as i32).unsigned_abs()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day7.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 344535);
    }

    #[test]
    fn input_known_answer_p2() {
        let result = run_p2(&input_generator(INPUT));

        assert_eq!(result, 95581659);
    }

    const EXAMPLE: [u16; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn example() {
        let fuel_cost = run(&EXAMPLE);

        assert_eq!(fuel_cost, 37);
    }

    #[test]
    fn example_p2() {
        let fuel_cost = run_p2(&EXAMPLE);

        assert_eq!(fuel_cost, 168);
    }
}
