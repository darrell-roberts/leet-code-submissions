pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
  let mut result = vec![];
  let mut count = 0;
  for i in target.into_iter() {
    count += 1;
    let range_start = &count;
    for n in *range_start..=n {
      if n == i {
        result.push("Push".into());
        break;
      } else {
        result.push("Push".into());
        result.push("Pop".into());
        count += 1;
      }
    }
  }
  result
}

#[cfg(test)]
mod test {
  use crate::array_stack::build_array;

  #[test]
  fn test_build_array() {
    assert_eq!(
      vec!["Push", "Push", "Pop", "Push"],
      build_array(vec![1, 3], 3)
    );
    assert_eq!(vec!["Push", "Push", "Push"], build_array(vec![1, 2, 3], 3));
    assert_eq!(vec!["Push", "Push"], build_array(vec![1, 2], 4));
    assert_eq!(
      vec!["Push", "Pop", "Push", "Push", "Push"],
      build_array(vec![2, 3, 4], 4)
    );
  }
}
