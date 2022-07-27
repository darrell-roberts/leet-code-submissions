use std::collections::BTreeMap;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
  let mut result = vec![]; //String::new();

  for index in 0.. {
    let str_char_i = strs
            .iter()
            // .filter_map(|s| s.chars().nth(index))
            .filter_map(|s| s.as_bytes().get(index))
            .collect::<Vec<_>>();

    match str_char_i.get(0) {
      Some(&&first) => {
        if str_char_i.len() == strs.len()
          && str_char_i.iter().all(|&&c| c == first)
        {
          result.push(first);
        } else {
          return String::from_utf8(result).unwrap_or_default();
        }
      }
      None => return String::from_utf8(result).unwrap_or_default(),
    }
  }
  unreachable!()
}

#[allow(unused)]
pub fn longest_common_prefix_2(strs: Vec<String>) -> String {
  let mut freq_map: BTreeMap<usize, (char, usize)> = BTreeMap::new();

  for (i, c) in strs.iter().flat_map(|s| s.char_indices()) {
    match freq_map.get_mut(&i) {
      Some((prev_char, count)) => {
        if *prev_char == c {
          *count += 1;
        }
      }
      None => {
        freq_map.insert(i, (c, 1));
      }
    }
  }

  freq_map
    .values()
    .take_while(|&&(_, count)| count == strs.len())
    .map(|&(c, _)| c)
    .collect()
}

#[cfg(test)]
mod test {
  use super::{longest_common_prefix, longest_common_prefix_2};

  #[test]
  fn test_longest() {
    assert_eq!(
      "fl",
      longest_common_prefix(
        ["flower", "flow", "flight"]
          .into_iter()
          .map(String::from)
          .collect()
      )
    );
    assert_eq!(
      "",
      longest_common_prefix(
        ["dog", "racecar", "car"]
          .into_iter()
          .map(String::from)
          .collect()
      )
    );
    assert_eq!(
      "",
      longest_common_prefix([""].into_iter().map(String::from).collect())
    );
  }

  #[test]
  fn test2() {
    assert_eq!(
      "fl",
      longest_common_prefix_2(
        ["flower", "flow", "flight"]
          .into_iter()
          .map(String::from)
          .collect()
      )
    );
    assert_eq!(
      "",
      longest_common_prefix_2(
        ["dog", "racecar", "car"]
          .into_iter()
          .map(String::from)
          .collect()
      )
    );
    assert_eq!(
      "",
      longest_common_prefix_2([""].into_iter().map(String::from).collect())
    );
    assert_eq!(
      "a",
      longest_common_prefix_2(["a"].into_iter().map(String::from).collect())
    );
    assert_eq!(
      "",
      longest_common_prefix_2(
        ["", "b"].into_iter().map(String::from).collect()
      )
    );
    assert_eq!(
      "aa",
      longest_common_prefix_2(
        ["aaa", "aa", "aaa"].into_iter().map(String::from).collect()
      )
    );
    assert_eq!(
      "",
      longest_common_prefix_2(
        ["", "cbc", "c", "ca"]
          .into_iter()
          .map(String::from)
          .collect()
      )
    );
    assert_eq!(
      "a",
      longest_common_prefix_2(
        ["abca", "aba", "aaab"]
          .into_iter()
          .map(String::from)
          .collect()
      )
    );
  }
}
