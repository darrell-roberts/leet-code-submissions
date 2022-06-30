pub fn is_valid(s: String) -> bool {
  let mut open_parens = vec![];

  for c in s.chars() {
    match c {
      '(' | '{' | '[' => open_parens.push(c),
      ')' | '}' | ']' => match open_parens.pop() {
        Some('(') if c == ')' => continue,
        Some('{') if c == '}' => continue,
        Some('[') if c == ']' => continue,
        _ => return false,
      },
      _ => return false,
    };
  }
  open_parens.is_empty()
}

pub fn longest_valid_parentheses(s: String) -> i32 {
  let mut valid_count = 0;
  let mut parens = vec![-1];

  for (i, c) in (0_i32..).zip(s.chars()) {
    match c {
      '(' => parens.push(i),
      _ => {
        parens.pop();
        if parens.is_empty() {
          parens.push(i);
        } else {
          valid_count = valid_count.max(i - parens[parens.len() - 1]);
        }
      }
    };
  }

  valid_count
}

#[cfg(test)]
mod test {
  use crate::parentheses::{is_valid, longest_valid_parentheses};

  #[test]
  fn test_parentheses() {
    assert!(is_valid("()".into()));
    assert!(is_valid("()[]{}".into()));
    assert!(!is_valid("(]".into()));
    assert!(is_valid("{[]}".into()));
    assert!(!is_valid("([)]".into()));
    assert!(!is_valid("[".into()));
  }

  #[test]
  fn test_longest() {
    assert_eq!(2, longest_valid_parentheses("(()".into()));
    assert_eq!(4, longest_valid_parentheses(")()())".into()));
    assert_eq!(0, longest_valid_parentheses("".into()));
    assert_eq!(6, longest_valid_parentheses("()(())".into()));
    assert_eq!(0, longest_valid_parentheses(")(".into()));
    assert_eq!(2, longest_valid_parentheses("()(()".into()));
  }
}
