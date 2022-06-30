pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
  let mut v = nums;
  v.sort_unstable();
  v.dedup();

  for (&a, b) in v.iter().skip_while(|&&n| n < 1).zip(1..) {
    println!("{a} {b}");
    if a != b && b > 0 {
      return b;
    }
  }

  v.pop()
    .map(|d| d + 1)
    .map(|d| if d < 1 { 1 } else { d })
    .unwrap()
}

#[cfg(test)]
mod test {
  use crate::missing_positive::first_missing_positive;

  #[test]
  fn test_missing() {
    assert_eq!(2, first_missing_positive(vec![3, 4, -1, 1]));
    assert_eq!(3, first_missing_positive(vec![1, 2, 0]));
    assert_eq!(1, first_missing_positive(vec![-5]));
    assert_eq!(3, first_missing_positive(vec![0, 2, 2, 1, 1]));
  }
}
