mod aoc1;

use aoc::Puzzle;

fn main() {
    let puzzle = Puzzle::new(2021, 2);
    let input: String = aoc::get_input(puzzle, None).unwrap();
    let answer: String = aoc2::a::solve(input);
    println!("{}", answer);
}

trait Solver {
    type Item;

    fn parse(line: String) -> Self::Item;
    fn solve_one(inputs: Vec<Self::Item>) -> String;
    fn solve_two(inputs: Vec<Self::Item>) -> String {
        String::new()
    }
}

fn solve(puzzle: Puzzle, solver: Solver, part: Part, token: Option<String>) -> String {
    let input: String = aoc::get_input(puzzle, token).unwrap();
    let lines: Vec<String> = input.lines().collect();
    let inputs: Vec<Solver::Item> = lines.into_iter().map(|line| Solver::parse(line)).collect();
    solver::solve_one(li)
}

enum Part {
    One,
    Two,
}

struct Aoc2 {}

impl Solver for Aoc2 {
    type Item = aoc2::Step;

    fn parse(line: String) -> Self::Item {
        todo!()
    }

    fn solve_one(inputs: Vec<Self::Item>) -> String {
        todo!()
    }
}

mod aoc2 {
    use std::{convert::Infallible, io::Stdin, str::FromStr};

    struct Pos(usize, usize);

    impl Pos {
        pub fn forward(&mut self, n: usize) {
            self.0 += n;
        }

        pub fn down(&mut self, n: usize) {
            self.1 += n;
        }

        pub fn up(&mut self, n: usize) {
            self.1 -= n;
        }

        pub fn value(&self) -> usize {
            self.0 * self.1
        }
    }

    pub enum Step {
        Fwd(usize),
        Up(usize),
        Down(usize),
    }

    impl FromStr for Step {
        type Err = Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iter = s.split_whitespace();
            let dir: &str = iter.next().unwrap();
            let qnt: usize = iter.next().unwrap().parse().unwrap();
            match dir {
                "forward" => Ok(Step::Fwd(qnt)),
                "up" => Ok(Step::Up(qnt)),
                "down" => Ok(Step::Down(qnt)),
                _ => panic!("Bah!"),
            }
        }
    }

    pub mod a {
        use std::str::FromStr;

        use itertools::Itertools;

        use super::{Pos, Step};

        pub fn solve(input: String) -> String {
            let steps: Vec<Step> =
                input.lines().map(|line| Step::from_str(line).unwrap()).collect();
            let mut pos = Pos(0, 0);

            for step in steps {
                match step {
                    Step::Fwd(n) => pos.forward(n),
                    Step::Up(n) => pos.up(n),
                    Step::Down(n) => pos.down(n),
                }
            }

            pos.value().to_string()
        }
    }
}
