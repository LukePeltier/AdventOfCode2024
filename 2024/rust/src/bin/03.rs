advent_of_code::solution!(3);
use lazy_regex::regex;

pub fn part_one(input: &str) -> Option<u64> {
    let rx = regex!(r"mul\((\d{1,3}),(\d{1,3})\)");
    let mut product = 0;
    for (_, [leftnum, rightnum]) in rx.captures_iter(input).map(|c| c.extract()) {
        product += leftnum.parse::<u64>().unwrap() * rightnum.parse::<u64>().unwrap();
    }
    Some(product)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rx = regex!(r"(?s)don't\(\).*?(?:do\(\)|$)|mul\((\d{1,3}),(\d{1,3})\)");
    let result: usize = rx
        .captures_iter(input)
        .filter(|cap| cap.get(1).is_some())
        .map(|cap| cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap())
        .sum();

    Some(result.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
