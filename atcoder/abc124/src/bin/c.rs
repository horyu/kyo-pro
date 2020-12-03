#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let mut black_start = 0;
    let mut white_start = 0;
    for i in 0..s.len() {
        let is_black = (s[i] == '0') as i32;
        let is_white = (s[i] == '1') as i32;
        if i % 2 == 0 {
            black_start += is_white;
            white_start += is_black;
        } else {
            black_start += is_black;
            white_start += is_white;
        }
    }
    println!("{}", std::cmp::min(black_start, white_start));
}
