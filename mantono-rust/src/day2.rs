use std::str::FromStr;

use aoc::{Puzzle, Solver};

///        HOR    DEP    AIM
struct Pos(usize, usize, usize);

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

    pub fn inc_aim(&mut self, n: usize) {
        self.2 += n;
    }

    pub fn dec_aim(&mut self, n: usize) {
        self.2 -= n;
    }

    pub fn mult_aim_depth(&mut self, n: usize) {
        self.1 += self.2 * n;
    }
}

pub enum Step {
    Fwd(usize),
    Up(usize),
    Down(usize),
}

pub struct Aoc;

impl Solver for Aoc {
    type Item = Step;

    fn puzzle() -> aoc::Puzzle {
        Puzzle::new(2021, 2)
    }

    fn solve_one(inputs: &[Self::Item]) -> String {
        let mut pos = Pos(0, 0, 0);

        for step in inputs {
            match step {
                Step::Fwd(n) => pos.forward(*n),
                Step::Up(n) => pos.up(*n),
                Step::Down(n) => pos.down(*n),
            }
        }

        pos.value().to_string()
    }

    fn solve_two(inputs: &[Self::Item]) -> String {
        let mut pos = Pos(0, 0, 0);

        for step in inputs {
            match step {
                Step::Fwd(n) => {
                    pos.forward(*n);
                    pos.mult_aim_depth(*n)
                }
                Step::Up(n) => pos.dec_aim(*n),
                Step::Down(n) => pos.inc_aim(*n),
            }
        }

        pos.value().to_string()
    }
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let dir: &str = iter.next().unwrap();
        let qnt: usize = iter.next().unwrap().parse().unwrap();
        match dir {
            "forward" => Ok(Step::Fwd(qnt)),
            "up" => Ok(Step::Up(qnt)),
            "down" => Ok(Step::Down(qnt)),
            _ => Err(format!("Invalid direction: {}", dir)),
        }
    }
}
