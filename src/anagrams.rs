use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
  let mut anagram_map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
  for s in strs {
    let mut chars = s.clone().chars().collect::<Vec<_>>();
    chars.sort_unstable();
    match anagram_map.get_mut(&chars) {
      Some(cs) => cs.push(s),
      None => {
        anagram_map.insert(chars, vec![s]);
      }
    }
  }

  anagram_map.into_values().collect()
}

#[allow(unused)]
pub fn is_anagram(s: String, t: String) -> bool {
  if s.len() != t.len() {
    return false;
  }
  let mut char_count = HashMap::new();
  for c in s.chars() {
    match char_count.get_mut(&c) {
      Some(val) => *val += 1,
      None => {
        char_count.insert(c, 1);
      }
    }
  }

  for c in t.chars() {
    match char_count.get_mut(&c) {
      Some(val) => *val -= 1,
      None => return false,
    }
  }

  char_count.values().all(|&val| val == 0)
}

#[cfg(test)]
mod test {
  use crate::anagrams::group_anagrams;

  use super::is_anagram;

  #[test]
  fn test_group() {
    let mut expected =
      vec![vec!["bat"], vec!["tan", "nat"], vec!["eat", "tea", "ate"]];
    expected.sort();
    let mut result = group_anagrams(vec![
      "eat".into(),
      "tea".into(),
      "tan".into(),
      "ate".into(),
      "nat".into(),
      "bat".into(),
    ]);
    result.sort();
    assert_eq!(expected, result);
  }

  #[test]
  fn test_anagram() {
    assert!(is_anagram("anagram".into(), "nagaram".into()))
  }
}
