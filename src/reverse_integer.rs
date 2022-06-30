use std::iter::successors;

fn digits(x: i32) -> Vec<i32> {
  successors(Some((x, 0)), |(n, _)| {
    if *n == 0 {
      return None;
    }
    n.checked_rem(10)
      .and_then(|r| n.checked_div(10).map(|d| (d, r)))
  })
  .skip(1)
  .map(|(_, d)| d)
  .collect()
}

fn digits_to_i32(d: Vec<i32>) -> i32 {
  let mut multipliers = successors(Some(1), |n| Some(n * 10_i32));
  let mut total: i32 = 0;
  for (i, n) in d.into_iter().enumerate().rev() {
    if i == 0 && n == 0 {
      continue;
    }
    if let Some(m) = multipliers.next() {
      match m.checked_mul(n).and_then(|n| total.checked_add(n)) {
        Some(n) => {
          total = n;
        }
        None => return 0,
      }
    }
  }
  total
}

pub fn reverse(x: i32) -> i32 {
  digits_to_i32(digits(x))
}

#[cfg(test)]
mod test {
  use crate::reverse_integer::{digits, reverse};

  #[test]
  fn test_digits() {
    assert_eq!(vec![0, 5, 4, 0, 3, 2, 1], digits(1230450));
  }

  #[test]
  fn test_reverse() {
    assert_eq!(321, reverse(123));
    assert_eq!(-321, reverse(-123));
    assert_eq!(21, reverse(120));
  }
}
