advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let mut left_list: Vec<i64> = Vec::with_capacity(1000);
    let mut right_list: Vec<i64> = Vec::with_capacity(1000);

    for line in input.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        let left_num = nums[0].parse::<i64>().unwrap();
        let right_num = nums[1].parse::<i64>().unwrap();
        left_list.push(left_num);
        right_list.push(right_num);
    }

    left_list.sort();
    right_list.sort();

    let mut sum = 0;

    for (position, number) in left_list.iter().enumerate() {
        sum += (number - right_list.get(position).unwrap()).abs();
    }
    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut left_list: Vec<i64> = Vec::with_capacity(1000);
    let mut right_list: HashMap<i64, i64> = HashMap::new();

    for line in input.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        let left_num = nums[0].parse::<i64>().unwrap();
        let right_num = nums[1].parse::<i64>().unwrap();
        left_list.push(left_num);
        right_list
            .entry(right_num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut sum = 0;

    for number in left_list.iter() {
        sum += number * right_list.get(number).unwrap_or(&0);
    }
    Some(sum.try_into().unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
