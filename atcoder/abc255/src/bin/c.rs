#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        x: isize,
        a: isize,
        d: isize,
        n: isize,
    };
    let mut rs = (a - x).abs().min((a + d * (n - 1) - x).abs());
    if d == 0 {
        println!("{rs}");
        return;
    }
    let f = (x + d - a) as f64 / d as f64;
    for i in (f as isize - 1)..=(f as isize + 1) {
        if (0..n).contains(&i) {
            rs = rs.min((a + d * (i - 1) - x).abs());
        }
    }
    println!("{rs}");
}
