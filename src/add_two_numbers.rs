use crate::{from_array, ListNode};

pub fn add_two_numbers(
  l1: Option<Box<ListNode>>,
  l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let mut carry = 0;
  let mut digits = vec![];

  let mut l1_next = l1;
  let mut l2_next = l2;

  loop {
    let l1_val = l1_next.as_ref().map(|l| l.val);
    let l2_val = l2_next.as_ref().map(|l| l.val);

    let sum = carry + l1_val.unwrap_or_default() + l2_val.unwrap_or_default();

    carry = sum / 10;
    digits.push(sum % 10);

    l1_next = l1_next.and_then(|l| l.next);
    l2_next = l2_next.and_then(|l| l.next);

    if l1_next.is_none() && l2_next.is_none() {
      if carry > 0 {
        digits.push(carry);
      }
      break;
    }
  }

  from_array(&digits)
}

#[cfg(test)]
mod test {
  use crate::add_two_numbers::add_two_numbers;
  use crate::{from_array, to_array};

  #[test]
  fn test_add_two_numbers() {
    let l1 = from_array(&[2, 4, 3]);
    let l2 = from_array(&[5, 6, 4]);
    assert_eq!(vec![7, 0, 8], to_array(add_two_numbers(l1, l2)));

    let l1 = from_array(&[9, 9, 9, 9, 9, 9, 9]);
    let l2 = from_array(&[9, 9, 9, 9]);

    assert_eq!(
      vec![8, 9, 9, 9, 0, 0, 0, 1],
      to_array(add_two_numbers(l1, l2))
    );

    let l1 = from_array(&[9]);
    let l2 = from_array(&[1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);

    assert_eq!(
      vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
      to_array(add_two_numbers(l1, l2))
    );
  }
}
