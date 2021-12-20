use std::str::FromStr;

#[derive(Clone)]
pub struct BitVec(Vec<bool>);

impl BitVec {
    pub fn new(size: usize) -> BitVec {
        let mut data = Vec::with_capacity(size);
        for i in 0..data.len() {
            data[i] = false;
        }
        BitVec(data)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> bool {
        self.0.get(index).unwrap().clone()
    }

    pub fn set(&mut self, index: usize, value: bool) {
        self.0[index] = value;
    }

    pub fn value(&self) -> u64 {
        assert!(self.len() <= 64);

        let mut value: u64 = 0;
        for i in 0..self.len() {
            let bit = self.0[i];
            value <<= 1;
            if bit {
                value += 1
            }
        }
        value
    }
}

impl FromStr for BitVec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output: Vec<bool> = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '1' => output.push(true),
                '0' => output.push(false),
                _ => return Err(format!("Invalid bit character: '{}'", c)),
            }
        }
        Ok(BitVec(output))
    }
}

impl IntoIterator for BitVec {
    type Item = bool;

    type IntoIter = BitVecIter;

    fn into_iter(self) -> Self::IntoIter {
        BitVecIter(0, self)
    }
}

pub struct BitVecIter(usize, BitVec);

impl Iterator for BitVecIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < self.1.len() {
            let value: Self::Item = self.1.get(self.0);
            self.0 += 1;
            Some(value)
        } else {
            None
        }
    }
}
