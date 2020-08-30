#![allow(dead_code)]

use std::collections::HashMap;

// T: O(n), S: O(n)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut map = HashMap::<i32, i32>::with_capacity(nums.len());
  for (i, v) in nums.into_iter().enumerate() {
    if map.contains_key(&(target - v)) {
      return vec![map[&(target - v)] as i32, i as i32];
    }
    map.insert(v, i as i32);
  }

  panic!("Not found!")
}


#[test]
fn test() {
  {
    let x = two_sum(vec![2, 7, 11, 15], 9);
    assert!(x.contains(&0));
    assert!(x.contains(&1));
  }
  {
    let x = two_sum(vec![3, 2, 4], 6);
    assert!(x.contains(&1));
    assert!(x.contains(&2));
  }
  {
    let x = two_sum(vec![3, 3], 6);
    assert!(x.contains(&0));
    assert!(x.contains(&1));
  }
}
