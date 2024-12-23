fn slice_except(slice: &[i64], exclude_index: usize) -> Vec<i64> {
    assert!(exclude_index < slice.len(), "index out of bounds");

    let mut result = Vec::with_capacity(slice.len() - 1);
    result.extend_from_slice(&slice[..exclude_index]);
    result.extend_from_slice(&slice[exclude_index + 1..]);
    result
}

fn is_safe(report: &[i64], checked_full: bool) -> bool {
    let mut ascending: Option<bool> = None;
    let mut prev_number: Option<i64> = None;
    let mut failing: bool = false;
    for num in report.iter() {
        let mut diff: Option<i64> = None;
        if prev_number.is_some() {
            diff = Some(num - prev_number.unwrap());
        }
        if diff.is_some() {
            let abs_diff = diff.unwrap().abs();
            if !(1..=3).contains(&abs_diff) {
                failing = true;
                break;
            }
            if ascending.is_some() {
                if (diff.unwrap() < 0) != ascending.unwrap() {
                    failing = true;
                    break;
                }
            } else {
                ascending = Some(diff.unwrap() < 0);
            }
        }
        prev_number = Some(*num);
    }

    if !failing {
        return true;
    }
    if !checked_full {
        for i in 0..report.len() {
            let dampened_slice = slice_except(report, i);
            if is_safe(dampened_slice.as_slice(), true) {
                return true;
            }
        }
    }
    // check -1
    false
}

pub fn solve(input: &str) -> u64 {
    let mut safe_reports = 0;

    for line in input.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        if is_safe(&nums, true) {
            safe_reports += 1;
        }
    }
    println!("Safe reports: {}", safe_reports);
    safe_reports
}
pub fn bonus(input: &str) -> u64 {
    let mut safe_reports = 0;

    for line in input.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        if is_safe(&nums, false) {
            safe_reports += 1;
        }
    }
    println!("Safe reports: {}", safe_reports);
    safe_reports
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::get_example_input;

    #[test]
    fn example_solve() {
        let result = solve(&get_example_input(2).unwrap());
        assert_eq!(2, result);
    }

    #[test]
    fn example_bonus() {
        let result = bonus(&get_example_input(2).unwrap());
        assert_eq!(4, result);
    }
}
