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

pub mod intervals {

  // #[derive(Debug)]
  // struct Graph {
  //   nodes: Vec<Node>,
  // }

  // impl Graph {
  //   fn new() -> Self {
  //     Self { nodes: vec![] }
  //   }

  //   fn add_node(&mut self, start: i32, end: i32) -> usize {
  //     let id = self.nodes.len();
  //     let node = Node {
  //       id,
  //       start,
  //       end,
  //       edges: vec![]
  //     };
  //     self.nodes.push(node);
  //     id
  //   }

  //   fn nodes_mut(&mut self) -> impl Iterator<Item = &mut Node> {
  //     self.nodes.iter_mut()
  //   }

  //   fn nodes(&self) -> impl Iterator<Item = &Node> {
  //     self.nodes.iter()
  //   }

  //   fn get_node(&self, id: usize) -> &Node {
  //     &self.nodes[id]
  //   }

  //   fn add_node_edge(&mut self, node1: usize, node2: usize) {
  //     if let Some(n) = self.nodes.get_mut(node1) {
  //       n.edges.push(node2);
  //     }
  //   }
  // }

  // #[derive(Debug)]
  // struct Node {
  //   id: usize,
  //   start: i32,
  //   end: i32,
  //   edges: Vec<usize>,
  // }

  // fn overlap(a: &Node, b: &Node) -> bool {
  //   a.start <= b.end && b.start <= a.end
  // }

  // fn build_graph(intervals: Vec<Vec<i32>>) -> Graph {
  //   let mut graph = Graph::new();
  //   for (a, b) in intervals.iter().zip(intervals.iter().skip(1)) {
  //     let node1 = graph.add_node(a[0], a[1]);
  //     let node2 = graph.add_node(b[0], b[1]);

  //     if overlap(graph.get_node(node1), graph.get_node(node2)) {
  //       println!("{a:?} overlaps {b:?} ids: {node1} {node2}");
  //       graph.add_node_edge(node1, node2);
  //     }

  //   }
  //   graph
  // }

  pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut merged: Vec<Vec<i32>> = vec![];
    let mut last_index = 0;

    for interval in intervals {
      if let Some(last_merged) = merged.get_mut(last_index) {
        if last_merged[1] < interval[0] {
          merged.push(interval);
          last_index += 1;
        } else {
          last_merged[1] = last_merged[1].max(interval[1]);
        }
      } else {
        merged.push(interval);
      }
    }

    merged
  }
}

#[cfg(test)]
mod test {
  use super::intervals::merge as merge_intervals;
  use super::{merge, merge_k_lists, merge_two_lists};
  use crate::from_array;

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

  #[test]
  fn test_merge_intervals() {
    let result =
      merge_intervals(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]);
    assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    assert_eq!(
      merge_intervals(vec![vec![1, 4], vec![4, 5]]),
      vec![vec![1, 5]]
    );
    assert_eq!(merge_intervals(vec![vec![1, 3]]), vec![vec![1, 3]]);
    assert_eq!(
      merge_intervals(vec![vec![1, 4], vec![5, 6]]),
      vec![vec![1, 4], vec![5, 6]]
    );
    assert_eq!(
      merge_intervals(vec![vec![1, 4], vec![0, 4]]),
      vec![vec![0, 4]]
    );
    assert_eq!(
      merge_intervals(vec![vec![1, 4], vec![0, 1]]),
      vec![vec![0, 4]]
    );
    assert_eq!(
      merge_intervals(vec![vec![1, 4], vec![2, 3]]),
      vec![vec![1, 4]]
    );
    assert_eq!(
      merge_intervals(vec![vec![1, 4], vec![0, 5]]),
      vec![vec![0, 5]]
    );
    assert_eq!(
      merge_intervals(vec![vec![1, 4], vec![0, 0]]),
      vec![vec![0, 0], vec![1, 4]]
    );
  }
}
