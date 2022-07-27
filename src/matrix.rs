pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
  for n in matrix.iter().flatten() {
    if &target == n {
      return true;
    }
  }
  false
}

#[cfg(test)]
mod test {
  use super::search_matrix;

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
}
