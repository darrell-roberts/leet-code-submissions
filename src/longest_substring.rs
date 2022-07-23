use std::{cmp::max, collections::HashMap};

/// Uses the "sliding window" algorithm to search within a contiguous structure.
pub fn length_of_longest_substring(s: String) -> i32 {
  let mut hmap = HashMap::new();
  let mut start = 0;
  let mut len = 0;
  for (i, c) in (0..).zip(s.chars()) {
    // check if we have a repeating character in the sliding window.
    if let Some(&n) = hmap.get(&c) {
      if n >= start {
        // move "left window to the right"
        start = n + 1;
      }
    }
    // select larger value between last window or incremented "right window"
    len = max(len, i - start + 1);
    hmap.insert(c, i);
  }
  len
}

#[cfg(test)]
mod test {
  use crate::length_of_longest_substring;

  #[test]
  fn test_length() {
    assert_eq!(3, length_of_longest_substring("abcabcbb".into()));
    assert_eq!(1, length_of_longest_substring("bbbbb".into()));
    assert_eq!(3, length_of_longest_substring("pwwkew".into()));
    assert_eq!(2, length_of_longest_substring("au".into()));
  }
}
