use std::ops::Index;

use crate::bits::BitVec;
use aoc::{Puzzle, Solver};
use itertools::Itertools;

#[derive(Clone)]

struct State([isize; 12]);

impl Default for State {
    fn default() -> Self {
        State([0; 12])
    }
}

impl State {
    pub fn get(&self, i: usize) -> bool {
        let value: &isize = self.0.get(i).unwrap();
        *value >= 0
    }

    pub fn apply(self, number: BitVec) -> State {
        let mut this = self.clone();
        for (i, bin) in number.into_iter().enumerate() {
            if bin {
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

struct BinState(Vec<BitVec>);

impl BinState {
    pub fn new(inputs: &Vec<BitVec>) -> BinState {
        BinState(inputs.clone())
    }

    fn get_bias(&self, i: usize) -> isize {
        let mut sum: isize = 0;
        for val in &self.0 {
            if val.get(i) {
                sum += 1;
            } else {
                sum -= 1;
            }
        }
        sum
    }

    fn get_state(&self, i: usize) -> bool {
        self.get_bias(i) >= 0
    }

    fn filter_state(self, state: bool, i: usize) -> BinState {
        let len: usize = self.0.get(0).unwrap().len();
        let i: usize = i % len;
        let filtered: Vec<BitVec> =
            self.0.into_iter().filter(|bin: &BitVec| bin.get(i) == state).collect();
        BinState(filtered)
    }

    fn completed(&self) -> bool {
        self.0.len() == 1
    }

    pub fn value(&self) -> usize {
        self.0.get(0).unwrap().value() as usize
    }
}

pub struct Aoc;

impl Solver for Aoc {
    type Item = BitVec;

    fn puzzle() -> aoc::Puzzle {
        Puzzle::new(2021, 3)
    }

    fn solve_one(inputs: &[Self::Item]) -> String {
        /*         let state = State::default();
        let state: State = inputs.into_iter().fold(state, |acc, bin| acc.apply(*bin));
        let product: usize = state.gamma() * state.epsilon();
        product.to_string() */
        String::from("foo")
    }

    //  0 0100
    // [1]1110
    // [1]0110
    // [1]0111
    // [1]0101
    //  0 1111
    //  0 0111
    // [1]1100
    // [1]0000
    // [1]1001
    //  0 0010
    //  0 1010
    // -----
    // 1 1 110
    // 1[0]110
    // 1[0]111
    // 1[0]101
    // 1 1 100
    // 1[0]000
    // 1 1 001
    // -----
    // 10[1]10
    // 10[1]11
    // 10[1]01
    // 10 0 00
    // -----
    // 101[1]0
    // 101[1]1
    // 101 0 1
    // -----
    // 1011 0
    // 1011[1]
    // -----
    // 10111
    fn solve_two(inputs: &[Self::Item]) -> String {
        let oxy_state = BinState::new(&inputs.to_vec());
        let oxy_state: BinState = div_conq(oxy_state, 0, BitCriteria::MostCommon);
        let oxy_rating: usize = oxy_state.value();

        let co2_state = BinState::new(&inputs.to_vec());
        let co2_state: BinState = div_conq(co2_state, 0, BitCriteria::LeastCommon);
        let co2_rating: usize = co2_state.value();

        (oxy_rating * co2_rating).to_string()
    }
}

enum BitCriteria {
    MostCommon,
    LeastCommon,
}

fn div_conq(state: BinState, index: usize, criteria: BitCriteria) -> BinState {
    let s: bool = match criteria {
        BitCriteria::MostCommon => state.get_state(index),
        BitCriteria::LeastCommon => !state.get_state(index),
    };

    let new_state: BinState = state.filter_state(s, index);
    if new_state.completed() {
        new_state
    } else {
        div_conq(new_state, index + 1, criteria)
    }
}

#[cfg(test)]
mod tests {
    use aoc::Solver;

    use crate::day3::BitVec;

    use super::Aoc;

    #[test]
    fn test_day3() {
        let input = r#"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
        "#;

        let inputs: Vec<BitVec> = Aoc::parse_input(input.to_string());
        let result: String = Aoc::solve_two(&inputs);
        assert_eq!("230".to_string(), result)
    }
}
