pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  nums1.extend(nums2);
  nums1.sort_unstable();

  if nums1.len() % 2 == 0 {
    let median_index = nums1.len() / 2 - 1;
    let first = nums1[median_index];
    let second = nums1[median_index + 1];
    (first as f64 + second as f64) / 2_f64
  } else {
    nums1[nums1.len() / 2] as f64
  }
}

#[cfg(test)]
mod test {
  use crate::find_median_sorted_arrays;

  #[test]
  fn test_median() {
    assert_eq!(2.0_f64, find_median_sorted_arrays(vec![1, 3], vec![2]));
    assert_eq!(2.5_f64, find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
  }
}
