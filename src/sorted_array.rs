pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut find = (0_i32..).zip(nums.iter()).filter(|(_, &n)| n == target);

  match (find.next(), find.last()) {
    (Some((f, _)), Some((l, _))) => vec![f, l],
    (Some((f, _)), None) => vec![f, f],
    _ => vec![-1, -1],
  }
}

#[cfg(test)]
mod test {
  use crate::sorted_array::search_range;

  #[test]
  fn test_search() {
    assert_eq!(vec![3, 4], search_range(vec![5, 7, 7, 8, 8, 10], 8));
    assert_eq!(vec![0, 0], search_range(vec![1], 1));
  }
}
