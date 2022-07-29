use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
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

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
  nums.sort_unstable();
  let mut last_choice = 0;
  let mut last_distance = i32::MAX;

  for i in 0..nums.len() - 2 {
    let mut ptr_1 = i + 1;
    let mut ptr_2 = nums.len() - 1;

    while ptr_1 < ptr_2 {
      let sum = nums[i] + nums[ptr_1] + nums[ptr_2];
      let distance = sum - target;
      if distance.abs() < last_distance.abs() {
        last_choice = sum;
        last_distance = distance;
      }

      if sum > target {
        ptr_2 -= 1;
      } else {
        ptr_1 += 1;
      }
    }
  }

  last_choice
}

#[cfg(test)]
mod test {
  use crate::three_sum::three_sum;

  use super::three_sum_closest;

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

  #[test]
  fn test_three_sum_closest() {
    assert_eq!(2, three_sum_closest(vec![-1, 2, 1, -4], 1));
    assert_eq!(0, three_sum_closest(vec![0, 0, 0], 1));

    let test_list = vec![
      -43, 57, -71, 47, 3, 30, -85, 6, 60, -59, 0, -46, -40, -73, 53, 68, -82,
      -54, 88, 73, 20, -89, -22, 39, 55, -26, 95, -87, -57, -86, 28, -37, 43,
      -27, -24, -88, -35, 82, -3, 39, -85, -46, 37, 45, -24, 35, -49, -27, -96,
      89, 87, -62, 85, -44, 64, 78, 14, 59, -55, -10, 0, 98, 50, -75, 11, 97,
      -72, 85, -68, -76, 44, -12, 76, 76, 8, -75, -64, -57, 29, -24, 27, -3,
      -45, -87, 48, 10, -13, 17, 94, -85, 11, -42, -98, 89, 97, -66, 66, 88,
      -89, 90, -68, -62, -21, 2, 37, -15, -13, -24, -23, 3, -58, -9, -71, 0,
      37, -28, 22, 52, -34, 24, -8, -20, 29, -98, 55, 4, 36, -3, -9, 98, -26,
      17, 82, 23, 56, 54, 53, 51, -50, 0, -15, -50, 84, -90, 90, 72, -46, -96,
      -56, -76, -32, -8, -69, -32, -41, -56, 69, -40, -25, -44, 49, -62, 36,
      -55, 41, 36, -60, 90, 37, 13, 87, 66, -40, 40, -35, -11, 31, -45, -62,
      92, 96, 8, -4, -50, 87, -17, -64, 95, -89, 68, -51, -40, -85, 15, 50,
      -15, 0, -67, -55, 45, 11, -80, -45, -10, -8, 90, -23, -41, 80, 19, 29, 7,
    ];
    assert_eq!(255, three_sum_closest(test_list, 255));
  }
}
