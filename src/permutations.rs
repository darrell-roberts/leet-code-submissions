use std::collections::{BTreeSet, HashSet};

/*
I figured I could use pair swapping to generate all the permutations. My attempt worked in some
cases but was cyclical in others. After researching the topic I learned about Heaps algorithm which
correctly uses pair swapping to generate all permutations.

https://en.wikipedia.org/wiki/Heap%27s_algorithm#Details_of_the_algorithm

 */

/// My attempt.
pub fn permute_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
  if nums.len() == 1 {
    return vec![nums];
  }
  let mut rotating_nums = nums.clone();
  let mut permutations = HashSet::new();

  'main: loop {
    let mut swap_index = 0;
    while swap_index + 1 < nums.len() {
      rotating_nums.swap(swap_index, swap_index + 1);
      println!("swap: {rotating_nums:?}");
      if permutations.contains(&rotating_nums) {
        println!("Repeating {rotating_nums:?}");
        break 'main;
      } else {
        permutations.insert(rotating_nums.clone());
      }

      swap_index += 1;
    }
  }

  permutations.into_iter().collect()
}

/// Heaps algorithm.
pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
  let mut result = vec![nums.clone()];
  let mut counter = vec![0; nums.len()];
  let mut index = 1;
  while index < nums.len() {
    if counter[index] < index {
      if index % 2 == 0 {
        nums.swap(0, index);
      } else {
        nums.swap(counter[index], index);
      }
      result.push(nums.clone());
      counter[index] += 1;
      index = 1;
    } else {
      counter[index] = 0;
      index += 1;
    }
  }

  result
}

pub fn next_permutation(nums: &mut Vec<i32>) {
  let original = nums.clone();
  let mut result = BTreeSet::new();
  result.insert(original.clone());
  let mut counter = vec![0; nums.len()];
  let mut index = 1;
  while index < nums.len() {
    if counter[index] < index {
      if index % 2 == 0 {
        nums.swap(0, index);
      } else {
        nums.swap(counter[index], index);
      }
      result.insert(nums.clone());
      counter[index] += 1;
      index = 1;
    } else {
      counter[index] = 0;
      index += 1;
    }
  }

  dbg!(&result);

  let mut result_iter =
    result.into_iter().skip_while(|v| v != &original).skip(1);
  if let Some(v) = result_iter.next() {
    *nums = v;
  }
}

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let mut nums = nums;
  nums.sort_unstable();
  let mut used = vec![false; nums.len()];
  let mut result = vec![];
  backtrack(&nums, &mut result, &mut vec![], &mut used);
  result
}

fn backtrack(
  nums: &Vec<i32>,
  result: &mut Vec<Vec<i32>>,
  current: &mut Vec<i32>,
  used: &mut Vec<bool>,
) {
  if current.len() == nums.len() {
    result.push(current.clone());
  } else {
    for (i, &val) in nums.iter().enumerate() {
      if !(used[i] || i > 0 && !used[i - 1] && val == nums[i - 1]) {
        used[i] = true;
        current.push(val);
        backtrack(nums, result, current, used);
        current.pop();
        used[i] = false;
      }
    }
  }
}

#[cfg(test)]
mod test {
  use crate::permutations::{next_permutation, permute};

  use super::permute_unique;

  #[test]
  fn test_permutations() {
    let mut result = permute(vec![1, 2, 3]);
    result.sort_unstable();
    assert_eq!(
      vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
      ],
      result
    );
    result = permute(vec![0, 1]);
    result.sort_unstable();
    assert_eq!(vec![vec![0, 1], vec![1, 0]], result);
    assert_eq!(vec![vec![1]] as Vec<Vec<i32>>, permute(vec![1]));
    result = permute(vec![5, 4, 6, 2]);
    result.sort_unstable();
    let mut expected = vec![
      vec![5, 4, 6, 2],
      vec![5, 4, 2, 6],
      vec![5, 6, 4, 2],
      vec![5, 6, 2, 4],
      vec![5, 2, 4, 6],
      vec![5, 2, 6, 4],
      vec![4, 5, 6, 2],
      vec![4, 5, 2, 6],
      vec![4, 6, 5, 2],
      vec![4, 6, 2, 5],
      vec![4, 2, 5, 6],
      vec![4, 2, 6, 5],
      vec![6, 5, 4, 2],
      vec![6, 5, 2, 4],
      vec![6, 4, 5, 2],
      vec![6, 4, 2, 5],
      vec![6, 2, 5, 4],
      vec![6, 2, 4, 5],
      vec![2, 5, 4, 6],
      vec![2, 5, 6, 4],
      vec![2, 4, 5, 6],
      vec![2, 4, 6, 5],
      vec![2, 6, 5, 4],
      vec![2, 6, 4, 5],
    ];
    expected.sort_unstable();
    assert_eq!(expected, result);
  }

  #[test]
  fn test_next_permutation() {
    // let mut test = vec![6, 7, 5, 3, 5, 6, 2, 9, 1, 2, 7, 0, 9];
    // next_permutation(&mut test);

    let mut test = vec![3, 2, 1];
    next_permutation(&mut test);
    assert_eq!(vec![1, 2, 3], test);

    let mut test = vec![1, 2, 3];
    next_permutation(&mut test);
    assert_eq!(vec![1, 3, 2], test);

    let mut test = vec![1, 1, 5];
    next_permutation(&mut test);
    assert_eq!(vec![1, 5, 1], test);

    let mut test = vec![1, 3, 2];
    next_permutation(&mut test);
    assert_eq!(vec![2, 1, 3], test);
  }

  #[test]
  fn test_permutations_unique() {
    assert_eq!(
      vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
      permute_unique(vec![1, 1, 2])
    )
  }
}
