use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct MultiplesCandidate {
  val: i32,
  multiple: u32,
  multiple_value: i32,
}

fn find_multiples(n: i32, target: i32) -> HashSet<MultiplesCandidate> {
  let mut multiple_candidates = HashSet::new();

  for multiple in 1.. {
    let multiple_value = n * (multiple as i32);

    match multiple_value.cmp(&target) {
      Ordering::Greater => {
        break;
      }
      Ordering::Equal => {
        multiple_candidates.insert(MultiplesCandidate {
          val: n,
          multiple,
          multiple_value,
        });
        break;
      }
      Ordering::Less => {
        multiple_candidates.insert(MultiplesCandidate {
          val: n,
          multiple,
          multiple_value,
        });
      }
    }
  }
  multiple_candidates
}

fn multiple_to_array(multiple: &MultiplesCandidate) -> Vec<i32> {
  std::iter::repeat(multiple.val)
    .take(multiple.multiple as usize)
    .collect()
}

pub fn combination_sum_old(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  let mut result = HashSet::new();

  let multiples: Vec<MultiplesCandidate> = candidates
    .iter()
    .flat_map(|&n| find_multiples(n, target))
    .collect();

  for &n in candidates.iter() {
    for p in multiples.iter() {
      if n + p.multiple_value == target {
        let mut r = vec![n];
        let mut vs = multiple_to_array(p);
        r.append(&mut vs);
        r.sort();
        result.insert(r);
      }
    }
  }

  for p in multiples.iter() {
    if p.multiple_value == target {
      let mut vs = multiple_to_array(p);
      vs.sort();
      result.insert(vs);
    }
  }

  for &n in candidates.iter() {
    if n < target {
      let mut total: Vec<i32> = vec![n];
      for &x in candidates.iter() {
        if x < target {
          total.push(x);
        }
        if total.iter().sum::<i32>() == target {
          // let t = &total;/
          total.sort();
          result.insert(total);
          break;
        }
      }
    }
  }

  result.into_iter().collect()
}

pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  candidates.sort();
  candidates.dedup();

  let mut result = HashSet::new();
  let mut temp = vec![];
  find_numbers(&mut result, &candidates, &mut temp, target, 0);
  result.into_iter().collect()
}

fn find_numbers(
  result: &mut HashSet<Vec<i32>>,
  candidates: &[i32],
  temp: &mut Vec<i32>,
  sum: i32,
  index: usize,
) {
  match (sum, index) {
    (0, _) => {
      let mut v: Vec<i32> = temp.clone();
      v.sort();
      result.insert(v);
    }
    (n, i) if !n.is_negative() && i < candidates.len() => {
      let current_val = candidates[i];
      temp.push(current_val);
      find_numbers(result, candidates, temp, n - current_val, i);
      temp.pop();
      find_numbers(result, candidates, temp, n, i + 1);
    }
    (_, _) => (),
  }
}

#[cfg(test)]
mod test {
  use crate::combination_sum::{combination_sum, find_multiples};

  #[test]
  fn test_sum() {
    let mut result = combination_sum(vec![2, 3, 6, 7], 7);
    result.sort();
    for v in result.iter_mut() {
      v.sort();
    }
    assert_eq!(vec![vec![2, 2, 3], vec![7]], result);

    // assert_eq!(
    //   vec![vec![100, 100, 100, 100], vec![200, 200]] as Vec<Vec<i32>>,
    //   combination_sum(vec![100, 200, 4, 12], 400)
    // );

    let mut result = combination_sum(vec![2, 3, 5], 8);
    for v in result.iter_mut() {
      v.sort();
    }
    result.sort();
    assert_eq!(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]], result);
    assert_eq!(vec![] as Vec<Vec<i32>>, combination_sum(vec![2], 1));

    let mut expected = vec![
      vec![1, 1, 2, 5],
      vec![1, 1, 1, 1, 2, 3],
      vec![1, 2, 3, 3],
      vec![1, 2, 2, 2, 2],
      vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1, 2, 2],
      vec![1, 1, 1, 6],
      vec![1, 2, 6],
      vec![2, 2, 5],
      vec![3, 6],
      vec![3, 3, 3],
      vec![1, 1, 1, 1, 5],
      vec![1, 1, 1, 3, 3],
      vec![1, 1, 1, 2, 2, 2],
      vec![2, 2, 2, 3],
      vec![1, 1, 1, 1, 1, 1, 1, 2],
      vec![1, 1, 1, 1, 1, 1, 3],
      vec![1, 1, 2, 2, 3],
      vec![1, 1, 7],
      vec![1, 3, 5],
      vec![2, 7],
    ];

    expected.sort();
    let mut result = combination_sum(vec![2, 7, 6, 3, 5, 1], 9);
    result.sort();

    assert_eq!(expected, result);
  }

  // #[test]
  // fn test_power() {
  //   assert_eq!(3, highest_multiple(2, 8).multiple);
  // }
}
