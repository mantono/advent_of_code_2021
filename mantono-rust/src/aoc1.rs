fn parse(input: String) -> Vec<usize> {
    input
        .lines()
        .map(|line: &str| line.trim().parse::<usize>())
        .filter_map(|value| value.ok())
        .collect()
}

pub mod a {
    use super::parse;

    pub fn solve(input: String) -> String {
        let values: Vec<usize> = parse(input);

        let n: usize =
            zip_with_next(values).into_iter().filter(|(left, right)| left < right).count();

        n.to_string()
    }

    fn zip_with_next(inputs: Vec<usize>) -> Vec<(usize, usize)> {
        let mut shifted: Vec<usize> = inputs.clone();
        shifted.remove(0);
        inputs.into_iter().zip(shifted).collect()
    }
}

pub mod b {

    use super::parse;

    pub fn solve(input: String) -> String {
        let values: Vec<usize> = parse(input);
        let mut i = 0;
        let mut incr = 0;
        while i < values.len() - 3 {
            let left: usize = values[i..=(i + 2)].iter().sum();
            let right: usize = values[(i + 1)..=(i + 3)].iter().sum();
            if left < right {
                incr += 1;
            }
            i += 1;
        }
        incr.to_string()
    }

    #[cfg(test)]
    mod tests {
        use crate::aoc1::b::solve;

        #[test]
        fn test_window_of_three() {
            let input = r#"
                199
                200
                208
                210
                200
                207
                240
                269
                260
                263
                "#;

            let answer: String = solve(input.to_string());
            assert_eq!(String::from("5"), answer)
        }
    }
}
