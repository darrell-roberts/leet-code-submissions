use std::collections::HashSet;

fn validate<'a>(v: impl Iterator<Item = &'a char>) -> bool {
  let mut visited = HashSet::new();
  for c in v {
    match c {
      '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
        if visited.contains(c) {
          return false;
        } else {
          visited.insert(c);
        }
      }
      '.' => continue,
      _ => {
        return false;
      }
    }
  }
  true
}

fn validate_rows(board: &[Vec<char>]) -> bool {
  board.iter().all(|row| validate(row.iter()))
}

fn validate_columns(board: &[Vec<char>]) -> bool {
  (0..board.len())
    .map(|y| board.iter().flat_map(move |row| row.get(y)))
    .all(validate)
}

fn validate_box(board: &[Vec<char>]) -> bool {
  let col_max = board
    .get(0)
    .and_then(|row| row.len().checked_sub(3))
    .unwrap_or_default();
  let row_max = board.len().checked_sub(3).unwrap_or_default();

  (0..=col_max)
    .step_by(3)
    .flat_map(move |x| (0..=row_max).step_by(3).map(move |y| (x, y)))
    .map(|(x, y)| {
      board
        .iter()
        .skip(y)
        .take(3)
        .flat_map(move |row| row.iter().skip(x).take(3))
    })
    .all(validate)
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
  validate_rows(&board) && validate_columns(&board) && validate_box(&board)
}

#[cfg(test)]
mod test {
  use super::is_valid_sudoku;

  #[test]
  fn test_valid_sudoku() {
    let test_board = vec![
      vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert!(is_valid_sudoku(test_board));

    let test_board = vec![
      vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert!(!is_valid_sudoku(test_board));

    let test_board = vec![
      vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
      vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
      vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
      vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
      vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
      vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
      vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
    ];

    assert!(!is_valid_sudoku(test_board));

    let test_board = vec![
      vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
      vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
      vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
      vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];

    assert!(!is_valid_sudoku(test_board));
  }
}
