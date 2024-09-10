
/*
Two sum problem
*/

use std::vec;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if map.contains_key(&complement) {
            return vec![*map.get(&complement).unwrap() as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}
/*
Linked list two numbers problem
*/
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut p = &mut dummy_head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            carry = sum / 10;
            p.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            p = &mut p.as_mut().unwrap().next;
        }
        if carry > 0 {
            p.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        dummy_head.unwrap().next
    }

    #[allow(unused)]
    pub fn longest_substring_without_repeating_characters(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut max = 0;
        let mut start = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(j) = map.get(&c) {
                start = std::cmp::max(start, *j + 1);
            }
            max = std::cmp::max(max, i - start + 1);
            map.insert(c, i);
        }
        max as i32
    }
    #[allow(unused)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
      let mut nums = vec![nums1, nums2].concat();
      nums.sort();
      let n = nums.len();
      if n % 2 == 0 {
          (nums[n / 2 - 1] + nums[n / 2]) as f64 / 2.0
      } else {
          nums[n / 2] as f64
      }
    }
    /*
    given a string s, return the longest palindromic substring in s
     */
    pub fn longest_palindrome(s: String) -> String {
      let n = s.len();
      let mut start = 0;
      let mut end = 0;

      for i in 0..n {
        let len1 = expand_around_center(&s, i, i);
        let len2 = expand_around_center(&s, i, i + 1);
        let len = std::cmp::max(len1, len2);
        if len > end - start {
          start = i - (len - 1) / 2;
          end = i + len / 2;
        }
      }
      #[doc = "expand around center is a helper function to find the longest palindrome"]
      fn expand_around_center(s: &str, left: usize, right: usize) -> usize {
        let mut l = left as isize;
        let mut r = right;
        while l >= 0 && r < s.len() && s.chars().nth(l as usize) == s.chars().nth(r) {
          l -= 1;
          r += 1;
        }
        (r as isize - l - 1) as usize
      }
      return s[start..end + 1].to_string();
    }

    #[allow(unused)]
    pub fn reverse_integer(x: i32) -> i32 {
        let mut x = x;
        let mut rev = 0;
        while x != 0 {
            let pop = x % 10;
            x /= 10;
            if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && pop > 7) {
                return 0;
            }
            if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && pop < -8) {
                return 0;
            }
            rev = rev * 10 + pop;
        }
        return rev;
    }

    #[allow(unused)]
    pub fn atoi(s: String) -> i32 {
        let mut s = s.trim();
        let mut chars = s.chars();
        let mut sign: i32 = 1;
        if let Some(c) = chars.next() {
            if c == '-' {
                sign = -1;
                s = chars.as_str();
            } else if c == '+' {
                s = chars.as_str();
            } else {
                chars = s.chars();
            }
        }
        let mut result: i32 = 0;
        for c in s.chars() {
            if let Some(d) = c.to_digit(10) {
                result = result.checked_mul(10).and_then(|r| r.checked_add(d as i32)).unwrap_or_else(|| {
                    if sign == 1 {
                        i32::MAX
                    } else {
                        i32::MIN
                    }
                });
            } else {
                break;
            }
        }
        result = result.checked_mul(sign).unwrap_or_else(|| {
            if sign == 1 {
                i32::MAX
            } else {
                i32::MIN
            }
        });
        return result;
    }

    #[allow(unused)]
    fn longest_common_prefix(strs: Vec<String>) -> String {
      if strs.is_empty() {
        return "".to_string();
      }
      let mut prefix = strs[0].clone();
      for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
          prefix.pop();
        }
      }
      prefix
    }
}

#[cfg(test)]
mod test {

    use super::*;

  #[test]
  fn test_longest_common_prefix() {
    assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
  }
}