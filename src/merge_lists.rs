use std::mem::replace;

use crate::ListNode;

pub fn merge_k_lists(
  lists: Vec<Option<Box<ListNode>>>,
) -> Option<Box<ListNode>> {
  let mut dummy_head = ListNode::new(i32::MIN);
  for l in lists.into_iter().flatten().flat_map(|l| l.into_iter()) {
    dummy_head.push_ordered(l);
  }
  dummy_head.next
}

pub fn merge_two_lists(
  list1: Option<Box<ListNode>>,
  list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  match (list1, list2) {
    (Some(l1), Some(l2)) => {
      let mut l1_iter = l1.into_iter();
      let mut l2_iter = l2.into_iter();

      let mut val1 = &mut l1_iter.next();
      let mut val2 = &mut l2_iter.next();

      let mut dummy_head = ListNode::new(0);

      loop {
        match (&mut val1, &mut val2) {
          (Some(v1), Some(v2)) => {
            if v1.val < v2.val {
              if let Some(old) = replace(val1, l1_iter.next()) {
                dummy_head.push(old);
              }
            } else if let Some(old) = replace(val2, l2_iter.next()) {
              dummy_head.push(old);
            }
          }
          (Some(_), None) => {
            if let Some(old) = replace(val1, l1_iter.next()) {
              dummy_head.push(old);
            }
          }
          (None, Some(_)) => {
            if let Some(old) = replace(val2, l2_iter.next()) {
              dummy_head.push(old);
            }
          }
          (None, None) => break,
        }
      }
      dummy_head.next.take()
    }
    (Some(l1), None) => Some(l1),
    (None, Some(l2)) => Some(l2),
    (None, None) => None,
  }
}

#[allow(unused, clippy::ptr_arg)]
pub fn merge(nums1: &mut Vec<i32>, _m: i32, nums2: &mut Vec<i32>, _n: i32) {
  let mut nums2_iter = nums2.iter().cloned();
  for n in nums1.iter_mut() {
    if *n == 0 {
      if let Some(n2) = nums2_iter.next() {
        *n = n2;
      }
    }
  }
  nums1.sort_unstable();
}

#[cfg(test)]
mod test {
  use crate::from_array;
  use crate::merge_lists::{merge_k_lists, merge_two_lists};

  use super::merge;

  #[test]
  fn test_merge() {
    let result = merge_k_lists(vec![
      from_array(&[1, 4, 5]),
      from_array(&[1, 3, 4]),
      from_array(&[2, 6]),
    ]);
    let expected = from_array(&[1, 1, 2, 3, 4, 4, 5, 6]);
    assert_eq!(result, expected);

    let result =
      merge_k_lists(vec![from_array(&[2]), from_array(&[]), from_array(&[-1])]);
    let expected = from_array(&[-1, 2]);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_merge_two() {
    let list_1 = from_array(&[1, 2, 4]);
    let list_2 = from_array(&[1, 3, 4]);
    let result = merge_two_lists(list_1, list_2);
    dbg!(&result);
    let expected = from_array(&[1, 1, 2, 3, 4, 4]);
    assert_eq!(result, expected);

    let list_1 = from_array(&[]);
    let list_2 = from_array(&[]);
    let result = merge_two_lists(list_1, list_2);
    let expected = from_array(&[]);
    assert_eq!(result, expected);

    let list_1 = from_array(&[]);
    let list_2 = from_array(&[0]);
    let result = merge_two_lists(list_1, list_2);
    let expected = from_array(&[0]);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_iter() {
    let test_list = from_array(&[1, 2, 3]);
    for l in test_list.unwrap().into_iter() {
      dbg!(&l);
    }
  }

  #[test]
  fn test_merge_sorted_array() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];

    merge(&mut nums1, 3, &mut nums2, 3);

    assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);

    let mut nums1 = vec![1];
    let mut nums2 = vec![];

    merge(&mut nums1, 1, &mut nums2, 0);

    assert_eq!(vec![1], nums1);

    let mut nums1 = vec![1];
    let mut nums2 = vec![];

    merge(&mut nums1, 1, &mut nums2, 0);

    assert_eq!(vec![1], nums1);
  }
}
