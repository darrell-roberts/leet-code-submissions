pub fn max_area(height: Vec<i32>) -> i32 {
  let mut area = 0;
  let mut left = 0;
  let mut right = height.len() - 1;

  while left < right {
    let width = right - left;
    let left_height = height[left];
    let right_height = height[right];
    let current_area =
      left_height.min(right_height) * i32::try_from(width).unwrap();
    area = area.max(current_area);
    if left_height <= right_height {
      left += 1;
    } else {
      right -= 1;
    }
  }
  area
}

#[cfg(test)]
mod test {
  use crate::water_container::max_area;

  #[test]
  fn test_max_area() {
    assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    assert_eq!(17, max_area(vec![2, 3, 4, 5, 18, 17, 6]));
  }
}
