fn transpose(matrix: &mut Vec<Vec<i32>>) {
  let matrix_len = matrix.len();
  for i in 0..matrix_len {
    for j in i + 1..matrix_len {
      let tmp = matrix[j][i];
      matrix[j][i] = matrix[i][j];
      matrix[i][j] = tmp;
    }
  }
}

fn reflect(matrix: &mut Vec<Vec<i32>>) {
  let matrix_len = matrix.len();
  for v in matrix {
    for j in 0..matrix_len / 2 {
      v.swap(j, matrix_len - j - 1);
    }
  }
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
  transpose(matrix);
  reflect(matrix);
}

#[cfg(test)]
mod test {
  use crate::rotate_image::rotate;

  #[test]
  fn test_rotate() {
    let mut test_vec = vec![
      vec![5, 1, 9, 11],
      vec![2, 4, 8, 10],
      vec![13, 3, 6, 7],
      vec![15, 14, 12, 16],
    ];
    rotate(&mut test_vec);
    assert_eq!(
      vec![
        vec![15, 13, 2, 5],
        vec![14, 3, 4, 1],
        vec![12, 6, 8, 9],
        vec![16, 7, 10, 11],
      ],
      test_vec
    )
  }
}
