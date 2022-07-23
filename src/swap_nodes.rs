use crate::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  match head {
    Some(mut first) if first.next.is_some() => {
      let mut second = first.as_mut().next.take();
      first.as_mut().next = swap_pairs(second.as_mut().unwrap().next.take());
      second.as_mut().unwrap().next.replace(first);
      second
    }
    _ => head,
  }
}

pub fn swap_pairs_mine(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut dummy_head = Box::new(ListNode::new(0));

  if let Some(ln) = head {
    let mut ln_iter = ln.into_iter();

    loop {
      let first = ln_iter.next();
      let second = ln_iter.next();

      match (first, second) {
        (Some(mut a), Some(mut b)) => {
          let tail = b.next.take();
          a.next = tail;
          b.next = Some(a);
          dummy_head.push(b);
        }
        (Some(a), None) => dummy_head.push(a),
        (None, Some(b)) => dummy_head.push(b),
        (None, None) => break,
      }
    }
  }

  dummy_head.next
}

pub fn remove_nth_from_end(
  mut head: Option<Box<ListNode>>,
  n: i32,
) -> Option<Box<ListNode>> {
  let count = {
    let mut next_node = &head;
    let mut count = 0;
    while let Some(n) = &next_node {
      next_node = &n.next;
      count += 1;
    }
    count
  };
  let target = count - n - 1;
  if target == -1 {
    return head.unwrap().next;
  }

  let mut next_node = head.as_mut();
  let mut count = 0;

  while let Some(n) = next_node {
    if count == target {
      let child = n.next.take();
      n.next = child.unwrap().next;
      return head;
    } else {
      count += 1;
      next_node = n.next.as_mut();
    }
  }
  None
}

#[cfg(test)]
mod test {
  use crate::from_array;

  use super::{remove_nth_from_end, swap_pairs};

  #[test]
  fn test_swap() {
    assert_eq!(
      from_array(&[2, 1, 4, 3]),
      swap_pairs(from_array(&[1, 2, 3, 4]))
    );
    assert_eq!(from_array(&[]), swap_pairs(from_array(&[])));
    assert_eq!(from_array(&[1]), swap_pairs(from_array(&[1])));
  }

  #[test]
  fn remove_nth() {
    let result = remove_nth_from_end(from_array(&[1, 2, 3, 4, 5]), 2);
    assert_eq!(result, from_array(&[1, 2, 3, 5]));
    assert_eq!(from_array(&[]), remove_nth_from_end(from_array(&[1]), 1));
    assert_eq!(
      from_array(&[2]),
      remove_nth_from_end(from_array(&[1, 2]), 2)
    );
  }
}
