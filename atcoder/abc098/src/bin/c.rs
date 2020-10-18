#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut left = 0usize;
    let mut right = (&s[1..]).iter().filter(|&&c| c == 'E').count();
    let mut rs = left + right;
    for i in 1..n {
        left += (s[i - 1] == 'W') as usize;
        right -= (s[i] == 'E') as usize;
        rs = std::cmp::min(rs, left + right);
    }
    println!("{}", rs);
}
