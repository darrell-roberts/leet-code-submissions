pub fn fib(n: i32) -> i32 {
  std::iter::successors(Some((0_i32, 1_i32)), |(n1, n2)| {
    n1.checked_add(*n2).map(|n3| (*n2, n3))
  })
  .map(|(n, _)| n)
  .nth(n.try_into().unwrap())
  .unwrap_or_default()
}

#[cfg(test)]
mod test {
  use crate::fibonacci::fib;

  #[test]
  fn test_fib() {
    assert_eq!(3, fib(4))
  }
}
