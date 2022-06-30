use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
  // if nums.iter().all(|&n| n == 0) {
  //   return vec![vec![0, 0, 0]];
  // }
  let mut triplet_set = HashSet::new();

  for (i, &n) in nums.iter().enumerate() {
    let mut set = HashSet::new();
    let current = 0 - n;
    for &n2 in nums.iter().skip(i + 1) {
      let inner = current - n2;
      if set.contains(&inner) {
        let mut triplet = vec![n, n2, inner];
        triplet.sort_unstable();
        triplet_set.insert(triplet);
      } else {
        set.insert(n2);
      }
    }
  }

  let mut r = triplet_set.into_iter().collect::<Vec<_>>();
  r.sort();
  r
}

#[cfg(test)]
mod test {
  use crate::three_sum::three_sum;

  #[test]
  fn test_three_sum() {
    assert_eq!(
      vec![vec![-1, -1, 2], vec![-1, 0, 1]],
      three_sum(vec![-1, 0, 1, 2, -1, -4])
    );
    assert_eq!(vec![] as Vec<Vec<i32>>, three_sum(vec![1, 2, -2, -1]));
    assert_eq!(
      vec![
        vec![-4, 0, 4],
        vec![-4, 1, 3], // missing
        vec![-3, -1, 4],
        vec![-3, 0, 3],
        vec![-3, 1, 2],
        vec![-2, -1, 3],
        vec![-2, 0, 2],
        vec![-1, -1, 2],
        vec![-1, 0, 1],
      ],
      three_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4])
    );
    assert_eq!(vec![] as Vec<Vec<i32>>, three_sum(vec![1, 2, -2, -1]));
    assert_eq!(
      vec![
        vec![-82, -11, 93],
        vec![-82, 13, 69],
        vec![-82, 17, 65],
        vec![-82, 21, 61],
        vec![-82, 26, 56],
        vec![-82, 33, 49],
        vec![-82, 34, 48],
        vec![-82, 36, 46],
        vec![-70, -14, 84],
        vec![-70, -6, 76],
        vec![-70, 1, 69],
        vec![-70, 13, 57],
        vec![-70, 15, 55],
        vec![-70, 21, 49],
        vec![-70, 34, 36],
        vec![-66, -11, 77],
        vec![-66, -3, 69],
        vec![-66, 1, 65],
        vec![-66, 10, 56],
        vec![-66, 17, 49],
        vec![-49, -6, 55],
        vec![-49, -3, 52],
        vec![-49, 1, 48],
        vec![-49, 2, 47],
        vec![-49, 13, 36],
        vec![-49, 15, 34],
        vec![-49, 21, 28],
        vec![-43, -14, 57],
        vec![-43, -6, 49],
        vec![-43, -3, 46],
        vec![-43, 10, 33],
        vec![-43, 12, 31],
        vec![-43, 15, 28],
        vec![-43, 17, 26],
        vec![-29, -14, 43],
        vec![-29, 1, 28],
        vec![-29, 12, 17],
        vec![-14, -3, 17],
        vec![-14, 1, 13],
        vec![-14, 2, 12],
        vec![-11, -6, 17],
        vec![-11, 1, 10],
        vec![-3, 1, 2],
      ] as Vec<Vec<i32>>,
      three_sum(vec![
        34, 55, 79, 28, 46, 33, 2, 48, 31, -3, 84, 71, 52, -3, 93, 15, 21, -43,
        57, -6, 86, 56, 94, 74, 83, -14, 28, -66, 46, -49, 62, -11, 43, 65, 77,
        12, 47, 61, 26, 1, 13, 29, 55, -82, 76, 26, 15, -29, 36, -29, 10, -70,
        69, 17, 49,
      ])
    );

    assert_eq!(vec![vec![0, 0, 0]], three_sum(vec![0, 0, 0, 0]));
  }
}
