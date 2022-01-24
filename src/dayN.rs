#[aoc_generator(dayN)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().next().unwrap().split(',').map(|n| n.parse::<u32>().unwrap()).collect()
}

#[aoc(dayN, part1)]
pub fn run(input: &[u32]) -> u64 {
    todo!()
}

// #[aoc(dayN, part2)]
// pub fn run_p2(input: &[u32]) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/dayN.txt");

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
