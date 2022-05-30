#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        abc: [usize; 3]
    };
    let gcd = abc[0].gcd(&abc[1]).gcd(&abc[2]);
    let rs = abc.into_iter().map(|x| x / gcd - 1).sum::<usize>();
    println!("{rs}");
}
