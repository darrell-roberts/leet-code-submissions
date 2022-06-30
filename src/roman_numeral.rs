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

#[cfg(test)]
mod test {
  use crate::roman_numeral::int_to_roman;

  #[test]
  fn test_to_roman() {
    assert_eq!("III", int_to_roman(3));
    assert_eq!("LVIII", int_to_roman(58));
    assert_eq!("XIV", int_to_roman(14));
    assert_eq!("XXVII", int_to_roman(27));
  }
}
