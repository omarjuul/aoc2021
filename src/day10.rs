#[aoc(day10, part1)]
pub fn run(input: &str) -> u64
{
    input.lines().map(corruption_score).sum()
}

fn corruption_score(line: &str) -> u64 {
    let mut to_close: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_closing(c) {
            if let Some(opening) = to_close.pop() {
                if closes(c, opening) {
                    continue;
                }
            }
            return corruption_score_for_char(c);
        } else {
            to_close.push(c);
        }
    }
    0
}

fn is_closing(c: char) -> bool {
    c == ')' || c == ']' || c == '}' || c == '>'
}

fn closes(closing: char, opening: char) -> bool {
    match opening {
        '(' => closing == ')',
        '[' => closing == ']',
        '{' => closing == '}',
        '<' => closing == '>',
        _ => panic!("expected one of {{(<[")
    }
}

fn corruption_score_for_char(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1_197,
        '>' => 25_137,
        _ => panic!("expected one of )]}}>")
    }
}

// #[aoc(day10, part2)]
// pub fn run_p2(input: &[u32]) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day10.txt");

    #[test]
    fn input_known_answer() {
        let result = run(INPUT);

        assert_eq!(result, 278475);
    }

    // #[test]
    // fn input_known_answer_p2() {
    //     let result = run_p2(&input_generator(INPUT));
    //
    //     assert_eq!(result, 0);
    // }

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn example() {
        let actual = run(EXAMPLE);

        assert_eq!(actual, 26397);
    }
    
    #[test]
    fn example1() {
        let actual = run("{([(<{}[<>[]}>{[]{[(<()>");
        assert_eq!(actual, 1197);
    }

    #[test]
    fn example2() {
        let actual = run("[[<[([]))<([[{}[[()]]]");
        assert_eq!(actual, 3);
    }

    #[test]
    fn example3() {
        let actual = run("[{[{({}]{}}([{[{{{}}([]");
        assert_eq!(actual, 57);
    }

    #[test]
    fn example4() {
        let actual = run("[<(<(<(<{}))><([]([]()");
        assert_eq!(actual, 3);
    }

    #[test]
    fn example5() {
        let actual = run("<{([([[(<>()){}]>(<<{{");
        assert_eq!(actual, 25137);
    }
}
