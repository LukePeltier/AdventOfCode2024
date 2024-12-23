use lazy_regex::Regex;

pub fn solve(input: &str) -> u64 {
    let rx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut product = 0;
    for (_, [leftnum, rightnum]) in rx.captures_iter(input).map(|c| c.extract()) {
        product += leftnum.parse::<u64>().unwrap() * rightnum.parse::<u64>().unwrap();
    }
    println!("Product: {}", product);
    product
}

pub fn bonus(input: &str) -> usize {
    let rx = Regex::new(r"(?s)don't\(\).*?(?:do\(\)|$)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result = rx
        .captures_iter(input)
        .filter(|cap| cap.get(1).is_some())
        .map(|cap| cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap())
        .sum();

    println!("Product: {}", result);
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::get_example_input;

    #[test]
    fn example_solve() {
        let result = solve(&get_example_input(3).unwrap());
        assert_eq!(161, result);
    }

    #[test]
    fn example_bonus() {
        let result = bonus(&get_example_input(3).unwrap());
        assert_eq!(48, result);
    }
}
