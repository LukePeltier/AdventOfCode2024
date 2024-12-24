use std::collections::HashMap;

pub fn solve(input: &str) -> i64 {
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
    println!("Sum: [{}]", sum);
    sum
}

pub fn bonus(input: &str) -> i64 {
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
    println!("Sum: [{}]", sum);
    sum
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::get_example_input;
    #[test]
    fn example_solve() {
        let result = solve(&get_example_input(1).unwrap());
        assert_eq!(11, result);
    }

    #[test]
    fn example_bonus() {
        let result = bonus(&get_example_input(1).unwrap());
        assert_eq!(31, result);
    }
}
