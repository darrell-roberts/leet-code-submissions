pub fn divide(dividend: i32, divisor: i32) -> i32 {
  let negative = (dividend.is_negative() || divisor.is_negative())
    && !(dividend.is_negative() && divisor.is_negative());

  let mut quotient: u32 = 0;
  let dividend: u64 = i64::from(dividend).abs().try_into().unwrap();
  let divisor: u64 = i64::from(divisor).abs().try_into().unwrap();
  let mut temp = 0;

  for i in (0..=31).rev() {
    let ls = divisor << i;
    if temp + ls <= dividend {
      temp += ls;
      quotient |= 1 << i as u32;
    }
  }

  i32::try_from(quotient)
    .map(|n| if negative { -n } else { n })
    .unwrap_or(if negative { i32::MIN } else { i32::MAX })
}

#[cfg(test)]
mod test {
  use super::divide;

  #[test]
  fn test_divide() {
    assert_eq!(5, divide(25, 5));
    assert_eq!(-2, divide(7, -3));
    assert_eq!(1, divide(-1, -1));
    assert_eq!(0, divide(1, 2));
    assert_eq!(1, divide(2, 2));
    assert_eq!(2147483647, divide(-2147483648, -1));
    assert_eq!(-2147483648, divide(-2147483648, 1));
    assert_eq!(-1073741824, divide(-2147483648, 2));
  }
}
