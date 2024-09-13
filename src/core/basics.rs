fn str_index(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    for i in 0..haystack.len() {
        if haystack[i] == needle[0] {
            let mut j = 0;
            while j < needle.len() && i + j < haystack.len() && haystack[i + j] == needle[j] {
                j += 1;
            }
            if j == needle.len() {
                return i as i32;
            }
        }
    }
    -1
}

#[doc = "
a bit flip is a change from 0 to 1 or from 1 to 0
"]
fn min_bit_flips(start: i32, goal: i32) -> i32 {
    let mut flips = 0;
    let mut xor = start ^ goal;
    while xor != 0 {
      flips += xor & 1;
      xor >>= 1;
    }
    if flips > 31 {
      return -1;
    }
    flips
}

#[doc =
"Trapping rain water
given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
"
]
fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n == 0 {
        return 0;
    }
    let mut left_max = vec![0; n];
    let mut right_max = vec![0; n];
    left_max[0] = height[0];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(height[i]);
    }
    right_max[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = right_max[i + 1].max(height[i]);
    }
    let mut water = 0;
    for i in 0..n {
        water += left_max[i].min(right_max[i]) - height[i];
    }
    water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_index() {
        assert_eq!(str_index("sadbutsad".to_string(), "sad".to_string()), 0);
        assert_eq!(str_index("aaaaa".to_string(), "bba".to_string()), -1);
        assert_eq!(str_index("".to_string(), "".to_string()), 0);
    }

    #[test]
    fn test_min_bit_flips() {
        assert_eq!(min_bit_flips(10, 7), 3);
        assert_eq!(min_bit_flips(2, 6), 1);
        assert_eq!(min_bit_flips(2, 7), 2);
        assert_eq!(min_bit_flips(0, 0), 0);
        assert_eq!(min_bit_flips(0, 1), 1);
        assert_eq!(min_bit_flips(0, 2), -1);
    }
}
