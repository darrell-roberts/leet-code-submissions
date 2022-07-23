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

#[cfg(test)]
mod test {
  use super::delete_duplicates;
  use crate::from_array;
  #[test]
  fn test_delete_dupes() {
    assert_eq!(from_array(&[1,2,3]), delete_duplicates(from_array(&[1,1,2,3,3])));
  }
}
