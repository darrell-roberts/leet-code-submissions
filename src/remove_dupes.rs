use crate::ListNode;

pub fn delete_duplicates(
  head: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  match head {
    Some(mut n) => {
      if let Some(next_node) = n.next.as_mut() {
        if next_node.val == n.val {
          return delete_duplicates(n.next.take())
        }
      }
      n.next = delete_duplicates(n.next);
      Some(n)
    },
    None => None
  }
}

#[allow(unused)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  nums.dedup();
  nums.len().try_into().unwrap()
}

#[allow(unused)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
  *nums = nums.iter().cloned().filter(|&n| n == val).collect::<Vec<i32>>();
  nums.len().try_into().unwrap()
}

#[allow(unused)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
  for (i, n) in nums.iter().enumerate() {
    match n.cmp(&target) {
      std::cmp::Ordering::Less => continue,
      std::cmp::Ordering::Equal => return i as i32,
      std::cmp::Ordering::Greater => return i as i32,
    }
  }

  nums.len().try_into().unwrap()
}

#[cfg(test)]
mod test {
  use super::{delete_duplicates, search_insert};
  use crate::from_array;
  #[test]
  fn test_delete_dupes() {
    assert_eq!(from_array(&[1,2,3]), delete_duplicates(from_array(&[1,1,2,3,3])));
  }

  #[test]
  fn test_search_insert() {
    assert_eq!(2, search_insert(vec![1,3,5,6], 5));
    assert_eq!(1, search_insert(vec![1,3,5,6], 2));
    assert_eq!(4, search_insert(vec![1,3,5,6], 7));
  }
}
