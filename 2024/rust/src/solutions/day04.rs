struct Letter {
    value: char,
    row: u8,
    column: u8,
}

struct Puzzle {
    letters: Vec<Letter>,
}

impl Puzzle {}

pub fn solve(input: &str) -> usize {
    0
}

pub fn bonus(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::get_example_input;

    #[test]
    fn example_solve() {
        let result = solve(&get_example_input(4).unwrap());
        assert_eq!(18, result);
    }

    #[test]
    fn example_bonus() {
        let result = bonus(&get_example_input(4).unwrap());
        assert_eq!(9, result);
    }
}
