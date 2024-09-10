

/*
Three sum problem
Given an integer array of nums, return all the triplets
[i, j, k] such that i != j, i != k, and j != k, and
nums[i] + nums[j] + nums[k] == 0.
*/

struct Solution;

impl Solution {

  fn three_sum(&self, nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut result = Vec::new();
    nums.sort();
    for i in 0..nums.len() {
      if i > 0 && nums[i] == nums[i - 1] {
        continue;
      }
      let mut j = i + 1;
      let mut k = nums.len() - 1;
      while j < k {
        let sum = nums[i] + nums[j] + nums[k];
        if sum == 0 {
          result.push(vec![nums[i], nums[j], nums[k]]);
          while j < k && nums[j] == nums[j + 1] {
            j += 1;
          }
          while j < k && nums[k] == nums[k - 1] {
            k -= 1;
          }
          j += 1;
          k -= 1;
        } else if sum < 0 {
          j += 1;
        } else {
          k -= 1;
        }
      }
    }
    result
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_three_sum() {
    let s = Solution;
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = s.three_sum(nums);
    assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
  }
}