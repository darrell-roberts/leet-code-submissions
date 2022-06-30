use std::collections::HashMap;

#[allow(dead_code)]
/// Brute force
pub fn two_sum_old(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let v_iter = nums.iter().enumerate();

  for (i1, n1) in v_iter.clone() {
    for (i2, n2) in v_iter.clone() {
      if i1 == i2 {
        continue;
      }
      // println!("{n1} + {n2} = {target}?");
      if n1 + n2 == target {
        return vec![i1 as i32, i2 as i32];
      }
    }
  }
  vec![]
}

/// One pass hash table.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut hmap = HashMap::with_capacity(nums.len());

  for (i, &val) in nums.iter().enumerate() {
    if let Some(&v) = hmap.get(&(target - val)) {
      return vec![v as i32, i as i32];
    }
    hmap.insert(val, i);
  }

  unreachable!()
}

#[cfg(test)]
mod test {
  use crate::two_sum::two_sum;

  #[test]
  fn test_two_sum() {
    assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6));
    assert_eq!(vec![0, 2], two_sum(vec![-3, 4, 3, 90], 0));
  }
}
