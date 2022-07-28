use std::collections::HashMap;

#[allow(unused)]
pub fn single_number_mine(nums: Vec<i32>) -> i32 {
  let mut counts = HashMap::with_capacity(nums.len());
  for n in nums {
    if let Some(val) = counts.get_mut(&n) {
      *val += 1;
    } else {
      counts.insert(n, 1);
    }
  }

  counts
    .into_iter()
    .find(|&(_, c)| c != 2)
    .map(|(n, _)| n)
    .unwrap_or_default()
}

// Uses this technique BITWISE XOR
// https://www.geeksforgeeks.org/bitwise-operators-in-c-cpp/
pub fn single_number(nums: Vec<i32>) -> i32 {
  let mut response = 0;

  for n in nums {
    response ^= n;
  }

  response
}

#[cfg(test)]
mod test {
  use super::single_number;

  #[test]
  fn test_single_num() {
    assert_eq!(1, single_number(vec![2, 2, 1]));
    assert_eq!(4, single_number(vec![2, 2, 4, 5, 5, 1, 8, 8, 1]));
  }
}
