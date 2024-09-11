
struct Solution;

impl Solution {

  fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
      return 0;
    }
    for i in 0..nums.len() {
      if nums[i] == target {
        return i as i32;
      } else if nums[i] > target {
        return i as i32;
      }
    }
    nums.len() as i32
  }

  fn binary_search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    while left <= right {
      let mid = left + (right - left) / 2;
      if nums[mid as usize] == target {
        return mid;
      } else if nums[mid as usize] < target {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    left
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_search_insert() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
  }

  #[test]
  fn test_binary_search_insert() {
    assert_eq!(Solution::binary_search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::binary_search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::binary_search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(Solution::binary_search_insert(vec![1, 3, 5, 6], 0), 0);
  }
}