use aoc::{Puzzle, Solver};

#[derive(Clone, Copy)]
pub struct Bin([bool; 12]);

#[derive(Clone, Copy)]

struct State([isize; 12]);

impl Default for State {
    fn default() -> Self {
        State([0; 12])
    }
}

impl State {
    pub fn apply(self, number: Bin) -> State {
        let mut this = self.clone();
        for (i, bin) in number.0.iter().enumerate() {
            if *bin {
                this.0[i] += 1;
            } else {
                this.0[i] -= 1;
            }
        }
        this
    }

    fn gamma(&self) -> usize {
        let mut x: usize = 0;
        for i in 0..12 {
            let value = self.0[i];
            x <<= 1;
            if value > 0 {
                x += 1
            }
        }
        x
    }

    fn epsilon(&self) -> usize {
        self.gamma() ^ 4095
    }
}

pub struct Aoc;

impl Solver for Aoc {
    type Item = Bin;

    fn puzzle() -> aoc::Puzzle {
        Puzzle::new(2021, 3)
    }

    fn parse(line: &str) -> Self::Item {
        let mut output: [bool; 12] = [false; 12];
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                output[i] = true;
            }
        }
        Bin(output)
    }

    fn solve_one(inputs: &[Self::Item]) -> String {
        let state = State::default();
        let state: State = inputs.into_iter().fold(state, |acc, bin| acc.apply(*bin));
        let product: usize = state.gamma() * state.epsilon();
        product.to_string()
    }

    fn solve_two(inputs: &[Self::Item]) -> String {
        let mut oxy_rating: Option<usize> = None;
        while let None = oxy_rating {
            for i in 0..12 {
                let ones: usize = inputs.iter().filter(|bin| bin.0[i]).count();
                let zeroes: usize = inputs.iter().filter(|bin| !bin.0[i]).count();
            }
        }

        let mut co2_rating: Option<usize> = None;
    }
}
