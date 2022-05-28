#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
        c: isize,
    };
    let mut rs = std::isize::MAX;
    for i in 0..=((n / a).min(9999)) {
        let nn = n - a * i;
        for j in 0..=((nn / b).min(9999 - i)) {
            let k = (nn - b * j) / c;
            if i + j + k <= 9999 && b * j + c * k == nn {
                rs = rs.min(i + j + k);
            }
        }
    }
    println!("{rs}");
}
