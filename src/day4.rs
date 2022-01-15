#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<u8>, Vec<Board>) {
    let lines: Vec<_> = input.lines().collect::<Vec<_>>();

    let marks: Vec<_> = lines[0]
        .split(",")
        .map(|str| str.parse::<u8>().unwrap())
        .collect();
    let boards: Vec<_> = lines
        .into_iter()
        .skip(2)
        .collect::<Vec<_>>()
        .split(|&l| l == "")
        .into_iter()
        .map(|lines| {
            let board_tiles = lines
                .into_iter()
                .map(|&line| {
                    let vec = line
                        .split_whitespace()
                        .map(|str| str.parse::<u8>().ok())
                        .collect::<Vec<_>>();
                    let arr: [Option<u8>; 5] = vec.try_into().unwrap();
                    arr
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Board { rows: board_tiles }
        })
        .collect();

    (marks, boards)
}

#[aoc(day4, part1)]
pub fn run(input: &(Vec<u8>, Vec<Board>)) -> usize {
    let marks = &input.0;
    let mut boards = input.1.clone();

    let (winning_board, winning_mark) = determine_winning_board(marks, &mut boards);
    return boards[winning_board].get_score(winning_mark);
}

fn determine_winning_board(marks: &Vec<u8>, boards: &mut Vec<Board>) -> (usize, u8) {
    for &mark in marks {
        for (idx, board) in boards.iter_mut().enumerate() {
            board.mark_number(mark);
            if board.check_for_bingo() {
                return (idx, mark);
            }
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn run_to_lose(input: &(Vec<u8>, Vec<Board>)) -> usize {
    let marks = &input.0;
    let mut boards = input.1.clone();

    for _ in 1..boards.len() {
        let (winning_board, _) = determine_winning_board(marks, &mut boards);
        boards.remove(winning_board);
    }
    let (winning_board, winning_mark) = determine_winning_board(marks, &mut boards);
    return boards[winning_board].get_score(winning_mark);
}

#[derive(Clone, Copy)]
pub struct Board {
    rows: [[Option<u8>; 5]; 5],
}

impl Board {
    fn get_score(&self, just_marked: u8) -> usize {
        let sum: usize = self
            .rows
            .map(|r| {
                r.into_iter()
                    .filter_map(|opt| opt)
                    .map(|i| usize::from(i))
                    .into_iter()
                    .sum::<usize>()
            })
            .into_iter()
            .sum();
        sum * usize::from(just_marked)
    }
}

impl Board {
    fn check_for_bingo(&self) -> bool {
        for x in 0..5 {
            if (0..5)
                .map(|y| self.rows[x][y] == None)
                .into_iter()
                .all(|marked| marked)
            {
                return true;
            }
        }
        for y in 0..5 {
            if (0..5)
                .map(|x| self.rows[x][y] == None)
                .into_iter()
                .all(|marked| marked)
            {
                return true;
            }
        }
        false
    }

    fn mark_number(&mut self, to_mark: u8) {
        for x in 0..5 {
            for y in 0..5 {
                if self.rows[x][y] == Some(to_mark) {
                    self.rows[x][y] = None;
                }
            }
        }
    }
}

// #[aoc(day4, part2)]
// pub fn run_windowed_average(input: &[u32]) {}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day4.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 87456);
    }
}
