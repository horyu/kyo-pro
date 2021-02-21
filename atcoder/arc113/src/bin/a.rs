#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        k: usize
    };
    // 三乗根 Cube root
    let mut cbrt = 1usize;
    while (cbrt + 1).pow(3) <= k {
        cbrt += 1;
    }
    let mut rs = 0;
    // a <= b <= c で考える
    for a in 1..=cbrt {
        for b in a..=(k / a) {
            for c in b..=(k / a / b) {
                rs += if (a == b) && (b == c) {
                    1
                } else if (a == b) || (b == c) {
                    3
                } else {
                    6
                };
            }
        }
    }
    println!("{}", rs);
}
