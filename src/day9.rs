#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|n| n.chars().map(|f| f.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day9, part1)]
pub fn run(input: &[Vec<u32>]) -> u64 {
    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            sum += find_risk_level(input, x, y);
        }
    }
    sum
}

fn find_risk_level(input: &[Vec<u32>], x: usize, y: usize) -> u64 {
    let value = input[y][x];
    if y > 0 {
        let above = input[y - 1][x];
        if above <= value {
            return 0;
        }
    }
    if x > 0 {
        let left = input[y][x - 1];
        if left <= value {
            return 0;
        }
    }
    if y < input.len() - 1 {
        let below = input[y + 1][x];
        if below <= value {
            return 0;
        }
    }
    if x < input[y].len() - 1 {
        let right = input[y][x + 1];
        if right <= value {
            return 0;
        }
    }
    
    println!("low point {} found at ({},{})", value, x, y);
    value as u64 + 1
}

// #[aoc(day9, part2)]
// pub fn run_p2(input: &[u32]) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day9.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 448);
    }

    // #[test]
    // fn input_known_answer_p2() {
    //     let result = run_p2(&input_generator(INPUT));
    //
    //     assert_eq!(result, 0);
    // }

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn example() {
        let parsed = input_generator(&EXAMPLE);
        let actual = run(&parsed);

        assert_eq!(actual, 15);
    }

const EXAMPLE2: &str = "2199943210
3987894921
2856789892
8767896789
7899965678";

    #[test]
    fn example2() {
        let parsed = input_generator(&EXAMPLE2);
        let actual = run(&parsed);

        assert_eq!(actual, 26);
    }
}
