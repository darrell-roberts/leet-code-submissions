pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
  for n in matrix.iter().flatten() {
    if &target == n {
      return true;
    }
  }
  false
}

enum Direction {
  Down,
  Right,
}

fn consume_up(mut matrix: Vec<Vec<i32>>, result: &mut Vec<i32>) {
  let mut new_matrix = vec![];
  while let Some(mut row) = matrix.pop() {
    if !row.is_empty() {
      let val = row.remove(0);
      result.push(val);
      new_matrix.insert(0, row);
    }
  }
  consume_matrix(new_matrix, result);
}

fn consume_matrix(matrix: Vec<Vec<i32>>, result: &mut Vec<i32>) {
  if !matrix.is_empty() {
    let height = matrix.len() - 1;
    let mut new_matrix = vec![];
    let mut direction = Direction::Right;

    for (row_index, mut row) in matrix.into_iter().enumerate() {
      match direction {
        Direction::Right => {
          result.append(&mut row);
          direction = Direction::Down;
        },
        Direction::Down => {
          if row_index < height {
            if let Some(val) = row.pop() {
              result.push(val);
            }
            if !row.is_empty() {
              new_matrix.push(row);
            }
          } else {
            while let Some(val) = row.pop() {
              result.push(val)
            }
            consume_up(new_matrix, result);
            break;
          }
        }
      }
    }
  }
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
  let mut result = vec![];
  consume_matrix(matrix, &mut result);
  result
}

#[cfg(test)]
mod test {
  use super::{search_matrix, spiral_order};

  #[test]
  fn test_matrix() {
    let _result = search_matrix(
      vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
      ],
      5,
    );
  }

  #[test]
  fn test_spiral() {
    let result =
      spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);

    let result = spiral_order(vec![
      vec![1, 2, 3, 4],
      vec![5, 6, 7, 8],
      vec![9, 10, 11, 12],
    ]);

    assert_eq!(vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7], result);
    assert_eq!(vec![7, 9, 6], spiral_order(vec![vec![7], vec![9], vec![6]]));

    assert_eq!(
      vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10],
      spiral_order(vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16]
      ])
    );
  }
}
