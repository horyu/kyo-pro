#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        x: usize,
        mut aa: [usize; n]
    };
    aa.retain(|&a| a != x);
    let s = aa.iter().join(" ");
    println!("{}", s);
}
