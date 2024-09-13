

struct Solution;

impl Solution {
  #[allow(dead_code)]
  #[doc = "XOR Queries of a Subarray"]
  pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut xor = vec![0; arr.len() + 1];
    for i in 0..arr.len() {
      xor[i + 1] = xor[i] ^ arr[i];
    }
    let mut res = vec![];
    for q in queries {
      res.push(xor[q[0] as usize] ^ xor[q[1] as usize + 1]);
    }
    res
  }
}