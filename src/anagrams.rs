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

#[cfg(test)]
mod test {
  use crate::anagrams::group_anagrams;

  #[test]
  fn test_group() {
    assert_eq!(
      vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
      group_anagrams(vec![
        "eat".into(),
        "tea".into(),
        "tan".into(),
        "ate".into(),
        "nat".into(),
        "bat".into(),
      ])
    );
  }
}
