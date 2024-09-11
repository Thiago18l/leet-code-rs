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
