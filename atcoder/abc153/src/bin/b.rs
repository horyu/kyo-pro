#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        n: usize,
        aa: [usize; n],
    };
    println!(
        "{}",
        ["No", "Yes"][(aa.iter().sum::<usize>() >= h) as usize]
    );
}
