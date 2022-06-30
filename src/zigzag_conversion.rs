pub fn convert(s: String, num_rows: i32) -> String {
  if num_rows == 1 {
    return s;
  }

  let mut rows = vec![String::new(); s.len().min(num_rows as usize)];

  let mut current_row = 0;
  let mut descending = false;

  for c in s.chars() {
    rows[current_row].push(c);
    if current_row == 0 || current_row == num_rows as usize - 1 {
      descending = !descending;
    }
    if descending {
      current_row += 1;
    } else {
      current_row -= 1;
    }
  }

  rows.into_iter().fold(String::new(), |acc, s| acc + &s)
}

#[cfg(test)]
mod test {
  use crate::convert;

  #[test]
  fn test_zigzag() {
    assert_eq!(
      String::from("PAHNAPLSIIGYIR"),
      convert("PAYPALISHIRING".into(), 3)
    );
  }
}
