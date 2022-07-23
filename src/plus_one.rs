pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
  let mut carry = 0;
  let mut result = vec![];
  for (i, n) in digits.iter().rev().enumerate() {
    let sum = if i == 0 { n + 1 + carry } else { n + carry };
    carry = 0;
    if sum == 10 {
      carry = 1;
      result.insert(0, 0);
    } else {
      result.insert(0, sum);
    }
  }

  if carry > 0 {
    result.insert(0, carry);
  }

  result
}

pub fn add_binary(a: String, b: String) -> String {
  let mut a_binary = a.chars().rev();
  let mut b_binary = b.chars().rev();

  let mut result = vec![];
  let mut carry = false;

  loop {
    let a_val = a_binary.next();
    let b_val = b_binary.next();

    let val = match (a_val, b_val) {
      (Some('1'), Some('1')) => {
        if carry {
          '1'
        } else {
          carry = true;
          '0'
        }
      }
      (Some('0'), Some('1'))
      | (Some('1'), Some('0'))
      | (None, Some('1'))
      | (Some('1'), None) => {
        if carry {
          '0'
        } else {
          '1'
        }
      }
      (Some('0'), Some('0')) | (None, Some('0')) | (Some('0'), None) => {
        if carry {
          carry = false;
          '1'
        } else {
          '0'
        }
      }
      _ => break,
    };
    result.push(val);
  }

  if carry {
    result.push('1');
  }

  result.into_iter().rev().collect::<String>()
}

#[cfg(test)]
mod test {
  use super::{add_binary, plus_one};
  #[test]
  fn test_plus_one() {
    assert_eq!(vec![1, 2, 4], plus_one(vec![1, 2, 3]));
    assert_eq!(vec![1, 0], plus_one(vec![9]));
  }

  #[test]
  fn test_add_binary() {
    assert_eq!(
      String::from("100"),
      add_binary(String::from("11"), String::from("1"))
    );
    assert_eq!(
      String::from("10101"),
      add_binary(String::from("1010"), String::from("1011"))
    );
  }
}
