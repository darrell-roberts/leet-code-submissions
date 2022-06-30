use std::cmp::max;

// Longest palindrome.

pub fn longest_palindrome(s: String) -> String {
  if let Some(first_byte) = s.as_bytes().iter().next() {
    if s.as_bytes().iter().all(|b| b == first_byte) {
      return s;
    }
  }

  let mut start = 0;
  let mut end = 0;

  for i in 0..s.len() as isize {
    let len = max(expand_around(&s, i, i), expand_around(&s, i, i + 1));
    println!("{i} len {len} start: {start} end: {end}");
    if len > end - start {
      start = i - (len - 1) / 2;
      end = i + len / 2;
    }
    println!("{i} len {len} start: {start} end: {end}");
  }

  String::from_utf8_lossy(&s.as_bytes()[start as usize..(end + 1) as usize])
    .into()
}

fn expand_around(s: &str, mut left: isize, mut right: isize) -> isize {
  while left >= 0
    && right < s.len() as isize
    && s.as_bytes()[left as usize] == s.as_bytes()[right as usize]
  {
    left -= 1;
    right += 1;
  }
  right - left - 1
}

// Is palindrome number.

fn digits(x: i32) -> Vec<i32> {
  std::iter::successors(Some((x, 0)), |(n, _)| {
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

pub fn is_palindrome(x: i32) -> bool {
  if x.is_negative() {
    return false;
  }
  let d = digits(x);
  for (a, b) in d.iter().zip(d.iter().rev()) {
    if a != b {
      return false;
    }
  }
  true
}

#[cfg(test)]
mod test {
  use crate::palindrome::{is_palindrome, longest_palindrome};

  #[test]
  fn test_longest() {
    assert_eq!("aba", longest_palindrome("babad".into()));
    assert_eq!("bb", longest_palindrome("cbbd".into()));
    assert_eq!("ccc", longest_palindrome(("ccc").into()));
    assert_eq!("bb", longest_palindrome("bb".into()));
    assert_eq!("c", longest_palindrome("ac".into()));
    assert_eq!("cc", longest_palindrome("ccd".into()));
  }

  #[test]
  fn test_is_palindrome() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(10));
    assert!(!is_palindrome(1000021));
  }
}
