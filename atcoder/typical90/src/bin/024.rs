#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    let diff = std::iter::zip(aa, bb).fold(0, |acc, (a, b)| acc + a.abs_diff(b));
    let tf = diff <= k && (k - diff).is_even();
    println!("{}", ["No", "Yes"][tf as usize]);
}
