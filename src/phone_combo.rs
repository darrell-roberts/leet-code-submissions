use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
  let dial_pad = HashMap::from([
    ('2', vec!['a', 'b', 'c']),
    ('3', vec!['d', 'e', 'f']),
    ('4', vec!['g', 'h', 'i']),
    ('5', vec!['j', 'k', 'l']),
    ('6', vec!['m', 'n', 'o']),
    ('7', vec!['p', 'q', 'r', 's']),
    ('8', vec!['t', 'u', 'v']),
    ('9', vec!['w', 'x', 'y', 'z']),
  ]);

  let mut result = vec![];

  for (i, v) in digits.chars().flat_map(|c| dial_pad.get(&c)).enumerate() {
    for &c in v.iter() {
      if i == 0 {
        result.push(String::from(c));
      } else {
        let mut new_result = vec![];
        for c1 in result.iter() {
          if c1.len() == i {
            new_result.push(format!("{c1}{c}"));
          }
        }
        result.append(&mut new_result);
      }
    }
  }

  result
    .into_iter()
    .filter(|s| s.len() == digits.len())
    .collect()
}

#[cfg(test)]
mod test {
  use super::letter_combinations;

  #[test]
  fn test_combos() {
    let mut expected =
      vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
    expected.sort_unstable();
    let mut result = letter_combinations("23".into());
    result.sort_unstable();
    assert_eq!(expected, result);
    assert_eq!(vec![] as Vec<String>, letter_combinations("".into()));

    expected = vec![
      "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg",
      "bdh", "bdi", "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh",
      "cdi", "ceg", "ceh", "cei", "cfg", "cfh", "cfi",
    ];
    expected.sort_unstable();
    result = letter_combinations("234".into());
    result.sort_unstable();
    assert_eq!(expected, result);
  }
}
