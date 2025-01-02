use std::{collections::HashSet, process};

advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Guard {
    direction: Direction,
    current_position: (i32, i32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Location {
    row: i32,
    column: i32,
    obstacle: bool,
    visited: bool,
}

type PatrolMap = Vec<Location>;

fn get_max_dimensions(map: &PatrolMap) -> Result<usize, String> {
    let max_row = map.iter().map(|item| item.row).max().unwrap() + 1;
    let max_col = map.iter().map(|item| item.column).max().unwrap() + 1;

    let n = map.len();
    if usize::try_from(max_col).unwrap() * usize::try_from(max_row).unwrap() == n {
        Ok(max_row.try_into().unwrap())
    } else {
        Err(format!("Not a perfect square: [{}]", n))
    }
}

fn parse_input(input: &str) -> Result<(PatrolMap, Guard), &'static str> {
    let guard_chars: HashSet<char> = HashSet::from(['^', '>', 'v', '<']);
    let mut row_count = 0;
    let mut col_count = 0;
    let mut map = PatrolMap::new();
    let mut guard: Option<Guard> = None;

    for c in input.chars() {
        if c == '\n' {
            col_count = 0;
            row_count += 1;
            continue;
        }
        let mut location = Location {
            row: row_count,
            column: col_count,
            obstacle: c == '#',
            visited: false,
        };
        if guard_chars.contains(&c) {
            location.visited = true;
            guard = Some(Guard {
                direction: Direction::try_from(c).unwrap(),
                current_position: (location.row, location.column),
            });
        }
        map.push(location);
        col_count += 1;
    }
    Ok((map, guard.expect("No guard found")))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut puzzle_map, mut guard) = parse_input(input).unwrap();
    let max_dims = get_max_dimensions(&puzzle_map).unwrap().try_into().unwrap();

    while guard.current_position.0 < max_dims
        && guard.current_position.1 < max_dims
        && guard.current_position.0 >= 0
        && guard.current_position.1 >= 0
    {
        let current_row = guard.current_position.0;
        let current_col = guard.current_position.1;
        let current_direction = guard.direction;

        let mut desired_row = current_row;
        let mut desired_col = current_col;

        match current_direction {
            Direction::Up => {
                desired_row -= 1;
            }
            Direction::Right => {
                desired_col += 1;
            }
            Direction::Down => {
                desired_row += 1;
            }
            Direction::Left => {
                desired_col -= 1;
            }
        }

        let desired_location = match puzzle_map
            .iter_mut()
            .find(|loc| loc.row == desired_row && loc.column == desired_col)
        {
            Some(l) => l,
            None => {
                if desired_row < max_dims
                    && desired_row >= 0
                    && desired_col < max_dims
                    && desired_col >= 0
                {
                    eprintln!("Missing location: {} {}", desired_row, desired_row);
                    process::exit(1);
                }
                break;
            }
        };
        if desired_location.obstacle {
            guard.direction = match guard.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            continue;
        } else {
            guard.current_position = (desired_row, desired_col);
            desired_location.visited = true;
        }
    }
    let visited_locations_count = puzzle_map.into_iter().filter(|loc| loc.visited).count();

    Some(visited_locations_count.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
