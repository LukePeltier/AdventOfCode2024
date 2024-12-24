use lazy_regex::regex;

#[derive(Debug, Clone)]
struct Letter {
    value: char,
    row: u16,
    column: u16,
}

struct SortByRow(Letter);
struct SortByColumn(Letter);

impl PartialOrd for SortByRow {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.row.partial_cmp(&other.0.row)
    }
}
impl PartialEq for SortByRow {
    fn eq(&self, other: &Self) -> bool {
        self.0.row == other.0.row
    }
}
impl PartialOrd for SortByColumn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.column.partial_cmp(&other.0.column)
    }
}
impl PartialEq for SortByColumn {
    fn eq(&self, other: &Self) -> bool {
        self.0.column == other.0.column
    }
}

#[derive(Debug, Clone)]
struct Puzzle {
    letters: Vec<Letter>,
}

impl Puzzle {
    pub fn get_letter(&self, row: u16, column: u16) -> Option<&Letter> {
        self.letters
            .iter()
            .find(|&cell| cell.row == row && cell.column == column)
    }
    pub fn get_dimension(&self) -> Result<usize, String> {
        let n = self.letters.len();
        let root = (n as f64).sqrt() as usize;
        if root * root == n {
            Ok(root)
        } else {
            Err(format!("Not a perfect square: [{}]", n))
        }
    }
    pub fn get_row(&self, row: u16) -> Result<String, String> {
        let result = match self.get_dimension() {
            Ok(value) => u16::try_from(value).unwrap(),
            Err(e) => return Err(e.to_string()),
        };
        if row >= result {
            return Err(format!("Column [{}] is not in bounds", row));
        }
        let mut row_letters: Vec<SortByColumn> = self
            .letters
            .iter()
            .filter(|&letter| letter.row == row)
            .cloned()
            .map(SortByColumn)
            .collect();
        row_letters.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let row_string: String = row_letters
            .into_iter()
            .map(|letter| letter.0.value)
            .collect();
        Ok(row_string)
    }
    pub fn get_col(&self, column: u16) -> Result<String, String> {
        let result = match self.get_dimension() {
            Ok(value) => u16::try_from(value).unwrap(),
            Err(e) => return Err(e.to_string()),
        };
        if column >= result {
            return Err(format!("Column [{}] is not in bounds", column));
        }
        let mut col_letters: Vec<SortByRow> = self
            .letters
            .iter()
            .filter(|&letter| letter.column == column)
            .cloned()
            .map(SortByRow)
            .collect();
        col_letters.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let col_string: String = col_letters
            .into_iter()
            .map(|letter| letter.0.value)
            .collect();
        Ok(col_string)
    }
    pub fn get_lr_diag(&self, index: u16) -> Result<String, String> {
        let dimension = match self.get_dimension() {
            Ok(value) => u16::try_from(value).unwrap(),
            Err(e) => return Err(e.to_string()),
        };
        if index >= (2 * dimension - 1) {
            return Err(format!("Index [{}] is not in bounds", index));
        }
        let mut return_string = String::new();
        if index < dimension {
            // Start at top row, from right
            let mut current_coord: (u16, u16) = (0u16, dimension - 1 - index);
            while current_coord.0 < dimension && current_coord.1 < dimension {
                let current_letter = match self.get_letter(current_coord.0, current_coord.1) {
                    Some(l) => l,
                    None => return Err("Index out of bounds".to_string()),
                };
                return_string.push(current_letter.value);
                current_coord.0 += 1;
                current_coord.1 += 1;
            }
        } else {
            let mut current_coord: (u16, u16) = (index - dimension + 1, 0);
            while current_coord.0 < dimension && current_coord.1 < dimension {
                let current_letter = match self.get_letter(current_coord.0, current_coord.1) {
                    Some(l) => l,
                    None => return Err("Index out of bounds".to_string()),
                };
                return_string.push(current_letter.value);
                current_coord.0 += 1;
                current_coord.1 += 1;
            }
        }

        Ok(return_string)
    }
    pub fn get_rl_diag(&self, index: u16) -> Result<String, String> {
        let dimension = match self.get_dimension() {
            Ok(value) => u16::try_from(value).unwrap(),
            Err(e) => return Err(e.to_string()),
        };
        if index >= (2 * dimension - 1) {
            return Err(format!("Index [{}] is not in bounds", index));
        }
        let mut return_string = String::new();
        if index < dimension {
            // Start at top row, from right
            let mut current_coord: (u16, i32) = (0, index.into());
            while current_coord.0 < dimension && current_coord.1 >= 0 {
                let current_letter = match self
                    .get_letter(current_coord.0, u16::try_from(current_coord.1).unwrap())
                {
                    Some(l) => l,
                    None => return Err("Index out of bounds".to_string()),
                };
                return_string.push(current_letter.value);
                current_coord.0 += 1;
                current_coord.1 -= 1;
            }
        } else {
            let mut current_coord: (u16, i32) = ((index - dimension + 1), (dimension - 1).into());
            while current_coord.0 < dimension && current_coord.1 >= 0 {
                let current_letter = match self
                    .get_letter(current_coord.0, u16::try_from(current_coord.1).unwrap())
                {
                    Some(l) => l,
                    None => return Err("Index out of bounds".to_string()),
                };
                return_string.push(current_letter.value);
                current_coord.0 += 1;
                current_coord.1 -= 1;
            }
        }
        Ok(return_string)
    }
}

fn parse_input(input: &str) -> Result<Puzzle, &'static str> {
    let mut row_count = 0;
    let mut col_count = 0;
    let mut letters: Vec<Letter> = vec![];
    for c in input.chars() {
        if c == '\n' {
            col_count = 0;
            row_count += 1;
            continue;
        }
        letters.push(Letter {
            value: c,
            row: row_count,
            column: col_count,
        });
        col_count += 1;
    }
    Ok(Puzzle { letters })
}
pub fn solve(input: &str) -> usize {
    let puzzle = parse_input(input).unwrap();
    let dimension = puzzle.get_dimension().unwrap();
    let total_indexes = dimension * 2 - 1;
    let mut count = 0;
    let forward_rx = regex!(r"XMAS");
    let backward_rx = regex!(r"SAMX");
    for i in 0..total_indexes {
        if i < dimension {
            let row_string = puzzle.get_row(i.try_into().unwrap()).unwrap();
            count += forward_rx.find_iter(row_string.as_str()).count();
            count += backward_rx.find_iter(row_string.as_str()).count();
            let col_string = puzzle.get_col(i.try_into().unwrap()).unwrap();
            count += forward_rx.find_iter(col_string.as_str()).count();
            count += backward_rx.find_iter(col_string.as_str()).count();
        }
        let diag_lr_string = puzzle.get_lr_diag(i.try_into().unwrap()).unwrap();
        count += forward_rx.find_iter(diag_lr_string.as_str()).count();
        count += backward_rx.find_iter(diag_lr_string.as_str()).count();

        let diag_rl_string = puzzle.get_rl_diag(i.try_into().unwrap()).unwrap();
        count += forward_rx.find_iter(diag_rl_string.as_str()).count();
        count += backward_rx.find_iter(diag_rl_string.as_str()).count();
    }
    count
}

pub fn bonus(input: &str) -> usize {
    let puzzle = parse_input(input).unwrap();
    let dimension = puzzle.get_dimension().unwrap();
    let mut count = 0;
    for l in puzzle.letters.iter() {
        // Check if letter is an A
        if l.value != 'A' {
            continue;
        }
        // check if letter has room
        if l.row > 0
            && usize::from(l.row) < dimension - 1
            && l.column > 0
            && usize::from(l.column) < dimension - 1
        {
            let up_left: &Letter = puzzle.get_letter(l.row - 1, l.column - 1).unwrap();
            let up_right: &Letter = puzzle.get_letter(l.row - 1, l.column + 1).unwrap();
            let down_left: &Letter = puzzle.get_letter(l.row + 1, l.column - 1).unwrap();
            let down_right: &Letter = puzzle.get_letter(l.row + 1, l.column + 1).unwrap();

            let mut x_count = 0;

            if up_left.value == 'M' && down_right.value == 'S' {
                x_count += 1;
            }
            if up_left.value == 'S' && down_right.value == 'M' {
                x_count += 1;
            }

            if up_right.value == 'M' && down_left.value == 'S' {
                x_count += 1;
            }
            if up_right.value == 'S' && down_left.value == 'M' {
                x_count += 1;
            }

            if x_count == 2 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::get_example_input;
    fn get_test_puzzle() -> Puzzle {
        let input = &get_example_input(4).unwrap();
        let mut row_count = 0;
        let mut col_count = 0;
        let mut letters: Vec<Letter> = vec![];
        for c in input.chars() {
            if c == '\n' {
                col_count = 0;
                row_count += 1;
                continue;
            }
            letters.push(Letter {
                value: c,
                row: row_count,
                column: col_count,
            });
            col_count += 1;
        }
        Puzzle { letters }
    }

    #[test]
    fn get_row() {
        let result = get_test_puzzle().get_row(0);
        assert_eq!("MMMSXXMASM", result.unwrap());
    }

    #[test]
    fn get_column() {
        let puzzle = get_test_puzzle();
        println!("puzzle: [{:?}]", puzzle);
        let result = get_test_puzzle().get_col(2);
        assert_eq!("MAXAAASXMM", result.unwrap());
    }

    #[test]
    fn get_lr_diag_one() {
        let result = get_test_puzzle().get_lr_diag(3);
        assert_eq!("MMMX", result.unwrap());
    }
    #[test]
    fn get_lr_diag_two() {
        let result = get_test_puzzle().get_lr_diag(15);
        assert_eq!("SAMX", result.unwrap());
    }
    #[test]
    fn get_rl_diag_one() {
        let result = get_test_puzzle().get_rl_diag(3);
        assert_eq!("SAMM", result.unwrap());
    }
    #[test]
    fn get_rl_diag_two() {
        let result = get_test_puzzle().get_rl_diag(14);
        assert_eq!("ASAMX", result.unwrap());
    }

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
