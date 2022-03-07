#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    // 全部白にして右から黒を伸ばす
    let mut rs = s.iter().filter(|&&c| c == '#').count();
    let mut crr = rs;
    for c in s.into_iter().rev() {
        if c == '#' {
            crr -= 1;
        } else {
            crr += 1;
        }
        rs = rs.min(crr);
    }
    println!("{rs}");
}
