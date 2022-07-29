const NUMERALS: [(i32, &str); 13] = [
  (1, "I"),
  (4, "IV"),
  (5, "V"),
  (9, "IX"),
  (10, "X"),
  (40, "XL"),
  (50, "L"),
  (90, "XC"),
  (100, "C"),
  (400, "CD"),
  (500, "D"),
  (900, "CM"),
  (1000, "M"),
];

pub fn int_to_roman(mut num: i32) -> String {
  let mut result = String::new();

  while num > 0 {
    for &(i, c) in NUMERALS.iter().rev() {
      let mut div = num / i;
      num %= i;

      while div != 0 {
        result.push_str(c);
        div -= 1;
      }
    }
  }

  result
}

fn char_to_num(c: char) -> Option<u32> {
  match c {
    'I' => Some(1),
    'V' => Some(5),
    'X' => Some(10),
    'L' => Some(50),
    'C' => Some(100),
    'D' => Some(500),
    'M' => Some(1000),
    _ => None,
  }
}

pub fn roman_to_int(s: String) -> i32 {
  let mut char_iter = s.chars().filter_map(char_to_num);
  let mut response = 0;

  let mut char_one = char_iter.next();
  let mut char_two = char_iter.next();

  loop {
    match (char_one, char_two) {
      (Some(a), Some(b)) => {
        if a >= b {
          response += a;
          char_one = Some(b);
          char_two = char_iter.next();
        } else {
          response += b - a;
          char_one = char_iter.next();
          char_two = char_iter.next();
        }
      }
      (Some(a), None) => {
        response += a;
        break;
      }
      _ => break,
    }
  }

  response.try_into().expect("Not negative")
}

#[cfg(test)]
mod test {
  use crate::roman_numeral::int_to_roman;

  use super::roman_to_int;

  #[test]
  fn test_to_roman() {
    assert_eq!("III", int_to_roman(3));
    assert_eq!("LVIII", int_to_roman(58));
    assert_eq!("XIV", int_to_roman(14));
    assert_eq!("XXVII", int_to_roman(27));
  }

  #[test]
  fn test_from_roman() {
    assert_eq!(1994, roman_to_int("MCMXCIV".into()));
  }
}
