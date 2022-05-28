#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: u128,
        a: u128,
        b: u128,
    };
    let mut rs = n * (n + 1) / 2;
    let lcm = a.lcm(&b);
    rs += n / lcm * (lcm + (n / lcm) * lcm) / 2;
    rs -= n / a * (a + (n / a) * a) / 2;
    rs -= n / b * (b + (n / b) * b) / 2;
    println!("{rs}");
}
