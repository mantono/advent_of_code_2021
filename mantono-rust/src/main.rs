#[macro_use]
extern crate lazy_static;

mod bits;
mod day1;
mod day2;
mod day3;

fn main() {
    let answer: String = aoc::run::<day4::Aoc>().unwrap();
    println!("Answer: {}", answer);
}

mod day4 {
    use std::{collections, str::FromStr};

    use aoc::Solver;
    use itertools::Itertools;
    use regex::Regex;

    static TEST_ORDER: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";

    static ORDER: &str = "74,79,46,2,19,27,31,90,21,83,94,77,0,29,38,72,42,23,6,62,45,95,41,55,93,69,39,17,12,1,20,53,49,71,61,13,88,25,87,26,50,58,28,51,89,64,3,80,36,65,57,92,52,86,98,78,9,33,44,63,16,34,97,60,40,66,75,4,7,84,22,43,11,85,91,32,48,14,18,76,8,47,24,81,35,30,82,67,37,70,15,5,73,59,54,68,56,96,99,10";

    pub struct Aoc;

    impl Solver for Aoc {
        type Item = Board;

        fn parse_input<T: Into<String>>(input: T) -> Vec<Self::Item> {
            input
                .into()
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .skip(1)
                .inspect(|line| println!("{}", line))
                .chunks(5)
                .into_iter()
                // TODO: Something goes wrong here!!
                .map(|chunk| chunk.collect_vec().join("\n"))
                .inspect(|line| println!("{}", line))
                .map(|line| match Self::parse(&line) {
                    Ok(val) => val,
                    Err(_) => panic!("Unable to parse line: '{}'", line),
                })
                .collect()
        }

        fn solve_one(inputs: &[Self::Item]) -> String {
            let order: &str = if inputs.len() > 50 { ORDER } else { TEST_ORDER };
            loop {
                let mut inputs: Vec<Self::Item> = inputs.to_vec();
                for number in parse_order(order) {
                    for board in inputs.iter_mut() {
                        board.mark(number);
                        if board.has_bingo() {
                            let unmarked: u32 =
                                board.get_unmarked().iter().map(|val| **val as u32).sum::<u32>();
                            let score: u32 = unmarked * (number as u32);
                            return score.to_string();
                        }
                    }
                }
            }
        }
    }

    fn parse_order(order: &str) -> Vec<u16> {
        order.split(',').map(|val| val.parse::<u16>().unwrap()).collect_vec()
    }

    #[derive(Clone)]
    pub struct Board {
        grid: [u16; 25],
    }

    impl Board {
        pub fn mark(&mut self, num: u16) {
            for n in self.grid.iter_mut() {
                if *n == num {
                    *n += 100;
                    if *n >= 200 {
                        panic!("Illegal state")
                    }
                    return;
                }
            }
            panic!("This should never happen")
        }

        pub fn is_marked(&self, row: usize, col: usize) -> bool {
            self.grid.get(row * 5 + col).unwrap() >= &100u16
        }

        pub fn get_unmarked(&self) -> Vec<&u16> {
            self.grid.iter().filter(|n| *n < &100u16).collect_vec()
        }

        /// Row 0 => `0..5` or `0..=4`
        /// Row 1 => `5..10` or `5..=9`
        pub fn get_row(&self, row: usize) -> &[u16] {
            let start: usize = row * 5;
            let end: usize = start + 5;
            &self.grid[start..end]
        }

        pub fn has_bingo(&self) -> bool {
            for num in 0..5 {
                let bingo: bool = self.get_row(num).iter().all(|n| n >= &100);
                if bingo {
                    return true;
                }
            }
            false
        }
    }

    impl FromStr for Board {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            lazy_static! {
                static ref DIGIT: Regex = Regex::new(r"\d+").unwrap();
            }
            let numbers: Vec<u16> =
                DIGIT.find_iter(s).map(|d| d.as_str().parse::<u16>().unwrap()).collect_vec();
            let len: usize = numbers.len();

            let grid: [u16; 25] = match numbers.try_into() {
                Ok(grid) => grid,
                Err(_) => return Err(format!("Invalid vector, not 25 elements, was {}", len)),
            };

            Ok(Board { grid })
        }
    }

    #[cfg(test)]
    mod tests {
        use std::vec;

        use aoc::Solver;

        use super::{Aoc, Board};

        #[test]
        fn test_parse() {
            let input = r#"
            22 13 17 11  0
            8  2 23  4 24
            21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19

            "#;

            let board: Board = input.parse().unwrap();
            assert_eq!(vec![22, 13, 17, 11, 0], board.get_row(0));
            assert_eq!(vec![1, 12, 20, 15, 19], board.get_row(4));
        }

        static TEST_INPUT: &str = r#"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
            8  2 23  4 24
            21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19

            3 15  0  2 22
            9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
            2  0 12  3  7
        "#;

        #[test]
        fn test_parse_input() {
            let boards: Vec<Board> = Aoc::parse_input(TEST_INPUT);

            let b0 = boards.get(0).unwrap();
            assert_eq!(Some(&19), b0.get_row(4).get(4));

            let b1 = boards.get(1).unwrap();
            assert_eq!(Some(&23), b1.get_row(2).get(4));

            let b2 = boards.get(2).unwrap();
            assert_eq!(Some(&0), b2.get_row(4).get(1));
        }

        #[test]
        fn test_solve() {
            let anwser: String = Aoc::solve(TEST_INPUT);
            assert_eq!(String::from("4512"), anwser);
        }
    }
}
