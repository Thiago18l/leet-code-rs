mod core;
use core::core::{two_sum, ListNode, Solution};

fn main() {
    two_sum(vec![2, 7, 11, 15], 9);
    Solution::add_two_numbers(None, None);
    ListNode::new(1);
    let substring = Solution::longest_palindrome(String::from("babad"));
    println!("{}", substring);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let result = Solution::add_two_numbers(l1, l2);
        let unwrapped_result = result.unwrap();
        assert_eq!(unwrapped_result.val, 7);
        assert_eq!(unwrapped_result.next.unwrap().val, 0);
    }

    #[test]
    pub fn test_longest_substring_without_repeating_characters() {
        assert_eq!(
            Solution::longest_substring_without_repeating_characters("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::longest_substring_without_repeating_characters("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::longest_substring_without_repeating_characters("pwwkew".to_string()),
            3
        );
    }

    #[test]
    pub fn test_median_of_arrays_sorted() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    pub fn test_longest_palindrome() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "aba".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }
}
