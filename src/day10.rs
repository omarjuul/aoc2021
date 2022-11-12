#[aoc(day10, part1)]
pub fn run(input: &str) -> u64
{
    input.lines().map(|l| match determine_corruption(l) {
        Corruption::Corrupt(score) => score,
        _ => 0
    }).sum()
}

fn determine_corruption(line: &str) -> Corruption {
    let mut to_close: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_closing(c) {
            if let Some(opening) = to_close.pop() {
                if closes(c, opening) {
                    continue;
                }
            }
            return Corruption::Corrupt(corruption_score_for_char(c));
        } else {
            to_close.push(c);
        }
    }
    Corruption::ToComplete(to_close)
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

#[aoc(day10, part2)]
pub fn run_p2(input: &str) -> u64 {
    let to_complete = input.lines().filter_map(|l| match determine_corruption(l) {
        Corruption::Corrupt(_) => None,
        Corruption::ToComplete(a) => Some(a)
    });

    let mut scores: Vec<_> = to_complete.map(completion_score).collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn completion_score(mut to_complete: Vec<char>) -> u64 {
    let mut result = 0;
    while let Some(open) = to_complete.pop() {
        result *= 5;
        result += completion_score_for_opening(open);
    }
    result
}

fn completion_score_for_opening(opening: char) -> u64 {
    match opening {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("expected one of {{(<[")
    }
}

enum Corruption {
    Corrupt(u64),
    ToComplete(Vec<char>),
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day10.txt");

    #[test]
    fn input_known_answer() {
        let result = run(INPUT);

        assert_eq!(result, 278475);
    }

    #[test]
    fn input_known_answer_p2() {
        let result = run_p2(INPUT);

        assert_eq!(result, 3015539998);
    }

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


    #[test]
    fn example_p2() {
        let actual = run_p2(EXAMPLE);

        assert_eq!(actual, 288957);
    }


    #[test]
    fn example_p2_1() {
        let actual = run_p2("[({(<(())[]>[[{[]{<()<>>");
        assert_eq!(actual, 288957);
    }

    #[test]
    fn example_p2_2() {
        let actual = run_p2("[(()[<>])]({[<{<<[]>>(");
        assert_eq!(actual, 5566);
    }

    #[test]
    fn example_p2_3() {
        let actual = run_p2("(((({<>}<{<{<>}{[]{[]{}");
        assert_eq!(actual, 1480781);
    }

    #[test]
    fn example_p2_4() {
        let actual = run_p2("{<[[]]>}<{[{[{[]{()[[[]");
        assert_eq!(actual, 995444);
    }

    #[test]
    fn example_p2_5() {
        let actual = run_p2("<{([{{}}[<[[[<>{}]]]>[]]");
        assert_eq!(actual, 294);
    }
}