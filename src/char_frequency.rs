use std::collections::HashSet;

pub fn min_deletions(s: String) -> i32 {
  let mut frequency = [0_i32; 26];

  for c in s.as_bytes().iter() {
    frequency[(c - b'a') as usize] += 1;
  }

  let mut delete_count = 0;
  let mut seen_frequencies = HashSet::new();
  for mut freq in frequency.into_iter().filter(|&freq| freq > 0) {
    while freq > 0 && seen_frequencies.contains(&freq) {
      freq -= 1;
      delete_count += 1;
    }
    seen_frequencies.insert(freq);
  }

  delete_count
}

#[cfg(test)]
mod tests {
  use crate::char_frequency::min_deletions;

  #[test]
  fn test_min_deletions() {
    let test_string = "aab".into();
    assert_eq!(0, min_deletions(test_string));

    let test_string = "aaabbbcc".into();
    assert_eq!(2, min_deletions(test_string));

    let test_string = "ceabaacb".into();
    assert_eq!(2, min_deletions(test_string));

    let test_string = "abcabc".into();
    assert_eq!(3, min_deletions(test_string));

    let test_string = "bbcebab".into();
    assert_eq!(2, min_deletions(test_string));

    let test_string = "abcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwzabcdefghijklmnopqrstuvwxwz".into();
    assert_eq!(276, min_deletions(test_string));
  }
}
