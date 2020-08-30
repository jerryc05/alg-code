#![allow(dead_code)]

use std::cmp::max;

// T: O(n), S: O(1), only works with ranged chars
pub fn length_of_longest_substring(s: String) -> i32 {
  let mut map = [None; 128];
  let mut start = 0usize;
  let mut ans = 0i32;

  for (i, c) in s.bytes().enumerate() {
    match map[c as usize] {
      Some(map_v) if map_v >= start => {
        ans = max(ans,max(map_v - start + 1,i - map_v) as i32);
        start = map_v + 1;
      }
      _ => {
        ans = max(ans, (i - start + 1) as i32);
      }
    }
    map[c as usize] = Some(i);
  }

   max(ans, (s.len() - start) as i32)
}


#[test]
fn test() {
  assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
  assert_eq!(1, length_of_longest_substring("bbbbb".to_string()));
  assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
  assert_eq!(2, length_of_longest_substring("cdd".to_string()));
}