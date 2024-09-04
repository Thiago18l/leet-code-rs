mod core;

fn main() {
    core::core::two_sum(vec![2, 7, 11, 15], 9);
    core::core::Solution::add_two_numbers(None, None);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(core::core::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(core::core::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(core::core::ListNode {
            val: 2,
            next: Some(Box::new(core::core::ListNode {
                val: 4,
                next: Some(Box::new(core::core::ListNode {
                    val: 3,
                    next: None,
                })),
            })),
        }));
        let l2 = Some(Box::new(core::core::ListNode {
            val: 5,
            next: Some(Box::new(core::core::ListNode {
                val: 6,
                next: Some(Box::new(core::core::ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));
        let result = core::core::Solution::add_two_numbers(l1, l2);
        let unwrapped_result = result.unwrap();
        assert_eq!(unwrapped_result.val, 7);
        assert_eq!(unwrapped_result.next.unwrap().val, 0);
    }

    #[test]
    pub fn test_longest_substring_without_repeating_characters() {
        assert_eq!(core::core::Solution::longest_substring_without_repeating_characters("abcabcbb".to_string()), 3);
        assert_eq!(core::core::Solution::longest_substring_without_repeating_characters("bbbbb".to_string()), 1);
        assert_eq!(core::core::Solution::longest_substring_without_repeating_characters("pwwkew".to_string()), 3);
    }

    #[test]
    pub fn test_median_of_arrays_sorted() {
        assert_eq!(core::core::Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(core::core::Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
