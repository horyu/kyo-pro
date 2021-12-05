#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize, // <= 4 * 1e5
        aa: [usize; n] // <= 1e9 < 2**30
    };
    // dbg!(aa.iter().fold(0, |acc, a| acc ^ a));
    let xor_sum = aa.iter().fold(0, |acc, a| acc ^ a);
    let tf = aa.iter().any(|&a| xor_sum ^ a == 0) || n % 2 == 1;
    println!("{}", ["Lose", "Win"][tf as usize]);
}
