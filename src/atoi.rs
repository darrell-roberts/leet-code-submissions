use std::num::IntErrorKind::{NegOverflow, PosOverflow};

pub fn my_atoi(s: String) -> i32 {
  let mut negative = false;

  match s.trim().chars().take(2).collect::<String>().as_str() {
    "+-" => return 0,
    "-+" => return 0,
    "++" => return 0,
    "--" => return 0,
    _ => (),
  }

  let mut parsed_string = s
    .trim()
    .chars()
    .skip_while(|&c| {
      match c {
        '-' => negative = true,
        '+' => negative = false,
        _ => (),
      }
      c == '-' || c == '+'
    })
    .take_while(|c| c.is_digit(10))
    .collect::<String>();

  if negative {
    parsed_string.insert(0, '-');
  }

  match parsed_string.parse::<i32>() {
    Ok(v) => v,
    Err(e) => match &e.kind() {
      PosOverflow => i32::MAX,
      NegOverflow => i32::MIN,
      _ => 0,
    },
  }
}

#[cfg(test)]
mod test {
  use crate::atoi::my_atoi;

  #[test]
  fn test_atoi() {
    assert_eq!(42, my_atoi("42".into()));
    assert_eq!(-42, my_atoi("-42".into()));
    assert_eq!(-42, my_atoi("   -42".into()));
    assert_eq!(4193, my_atoi("4193 with words".into()));
    assert_eq!(-2147483648, my_atoi("-91283472332".into()));
    assert_eq!(0, my_atoi("words and 987".into()));
    assert_eq!(1, my_atoi("+1".into()));
    assert_eq!(0, my_atoi("+-12".into()));
  }
}
