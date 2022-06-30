/// https://leetcode.com/problems/add-two-numbers/
pub use add_two_numbers::add_two_numbers;
/// https://leetcode.com/problems/group-anagrams/
pub use anagrams::group_anagrams;
/// https://leetcode.com/problems/build-an-array-with-stack-operations/
pub use array_stack::build_array;
/// https://leetcode.com/problems/string-to-integer-atoi
pub use atoi::my_atoi;
/// https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/
pub use char_frequency::min_deletions;
/// https://leetcode.com/problems/combination-sum/
pub use combination_sum::combination_sum;
/// https://leetcode.com/problems/longest-common-prefix/
pub use common_prefix::longest_common_prefix;
/// https://leetcode.com/problems/course-schedule-ii
pub use course_schedule::find_order;
/// https://leetcode.com/problems/course-schedule-ii
pub use course_schedule_dag::find_order_dag;
/// https://leetcode.com/problems/excel-sheet-column-title/
pub use excel_sheet::convert_to_title;
/// https://leetcode.com/problems/fibonacci-number/
pub use fibonacci::fib;
/// https://leetcode.com/problems/longest-substring-without-repeating-characters/
pub use longest_substring::length_of_longest_substring;
/// https://leetcode.com/problems/median-of-two-sorted-arrays/
pub use median_arrays::find_median_sorted_arrays;
/// https://leetcode.com/problems/merge-k-sorted-lists/
pub use merge_lists::merge_k_lists;
pub use merge_lists::merge_two_lists;
/// https://leetcode.com/problems/first-missing-positive/
pub use missing_positive::first_missing_positive;
/// https://leetcode.com/problems/palindrome-number/
pub use palindrome::is_palindrome;
/// https://leetcode.com/problems/longest-palindromic-substring/
pub use palindrome::longest_palindrome;
/// https://leetcode.com/problems/valid-parentheses/
pub use parentheses::is_valid;
/// https://leetcode.com/problems/longest-valid-parentheses/
pub use parentheses::longest_valid_parentheses;
/// https://leetcode.com/problems/permutations
pub use permutations::permute;
/// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
pub use phone_combo::letter_combinations;
/// https://leetcode.com/problems/reverse-integer
pub use reverse_integer::reverse;
/// https://leetcode.com/problems/integer-to-roman/
pub use roman_numeral::int_to_roman;
/// https://leetcode.com/problems/rotate-image
pub use rotate_image::rotate;
/// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
pub use sorted_array::search_range;
/// https://leetcode.com/problems/3sum
pub use three_sum::three_sum;
/// https://leetcode.com/problems/two-sum/
pub use two_sum::two_sum;
/// https://leetcode.com/problems/container-with-most-water
pub use water_container::max_area;
/// https://leetcode.com/problems/zigzag-conversion
pub use zigzag_conversion::convert;

mod add_two_numbers;
mod anagrams;
mod array_stack;
mod atoi;
mod char_frequency;
mod combination_sum;
mod common_prefix;
mod course_schedule;
mod course_schedule_dag;
mod excel_sheet;
mod fibonacci;
mod longest_substring;
mod median_arrays;
mod merge_lists;
mod missing_positive;
mod palindrome;
mod parentheses;
mod permutations;
mod phone_combo;
mod reverse_integer;
mod roman_numeral;
mod rotate_image;
mod sorted_array;
mod swap_nodes;
mod three_sum;
mod two_sum;
mod water_container;
mod zigzag_conversion;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }

  fn append(self, val: i32) -> Self {
    Self {
      val,
      next: Some(Box::new(self)),
    }
  }

  /// Adds ListNode to the tail of this ListNode.
  fn push(&mut self, node: Box<ListNode>) {
    let mut next_node = &mut self.next;
    loop {
      match next_node {
        Some(n) => {
          next_node = &mut n.next;
        }
        None => {
          *next_node = Some(node);
          break;
        }
      }
    }
  }
}

pub struct IntoIter {
  inner: Option<Box<ListNode>>,
}

impl Iterator for IntoIter {
  type Item = Box<ListNode>;
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.take().map(|mut v| {
      self.inner = v.next.take();
      v
    })
  }
}

impl IntoIterator for ListNode {
  type Item = Box<ListNode>;
  type IntoIter = IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      inner: Some(Box::new(self)),
    }
  }
}

fn from_array(l: &[i32]) -> Option<Box<ListNode>> {
  let mut list_node: Option<Box<ListNode>> = None;

  for &i in l.iter().rev() {
    match list_node {
      Some(ln) => list_node = Some(Box::new(ln.append(i))),
      None => list_node = Some(Box::new(ListNode::new(i))),
    }
  }

  list_node
}

fn to_array(l: Option<Box<ListNode>>) -> Vec<i32> {
  let mut next = l;
  let mut digits = vec![];
  while let Some(n) = next {
    digits.push(n.val);
    next = n.next;
  }
  digits
}

#[cfg(test)]
mod test {
  use crate::{from_array, ListNode};

  #[test]
  fn test_push() {
    let test_list_node = from_array(&[1, 2, 3, 4, 5]);
    let test_add = Box::new(ListNode::new(6));

    if let Some(mut n) = test_list_node {
      n.push(test_add);
      dbg!(&n);
    }
  }
}