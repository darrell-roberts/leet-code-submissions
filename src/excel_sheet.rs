use std::iter::successors;

pub fn convert_to_title(column_number: i32) -> String {
  successors(Some((column_number, ' ')), |(n, _)| {
    n.checked_rem(26).and_then(|remainder| {
      let divisor = n.checked_div(26);
      if remainder == 0 {
        divisor.and_then(|div| div.checked_sub(1)).map(|n| (n, 'Z'))
      } else {
        remainder.checked_sub(1).and_then(|sub| {
          divisor.and_then(|d| ('A'..='Z').nth(sub as usize).map(|c| (d, c)))
        })
      }
    })
  })
  .take_while(|&(n, _)| n >= 0)
  .filter_map(|(_, c)| if c != ' ' { Some(c) } else { None })
  .collect::<String>()
  .chars()
  .rev()
  .collect::<String>()
}

/*
Haskell Prototyping.
λ> excel = reverse . fmap ((!!) ['A'..'Z'] . pred . (\n -> if n == 0 then 26 else n) . (`mod` 26)) . takeWhile (>1) . iterate (`div` 26)
λ> excel 2147483647
"FXSHRXW"
 */

#[cfg(test)]
mod test {
  use crate::excel_sheet::convert_to_title;

  #[test]
  fn test_column_title() {
    assert_eq!("A", convert_to_title(1));
    assert_eq!("B", convert_to_title(2));
    assert_eq!("Z", convert_to_title(26));
    assert_eq!("AB", convert_to_title(28));
    assert_eq!("AZ", convert_to_title(52));
    assert_eq!("ZY".to_string(), convert_to_title(701));
    assert_eq!("FXSHRXW", convert_to_title(2147483647));
  }
}
