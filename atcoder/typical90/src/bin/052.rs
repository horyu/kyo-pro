#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aaa: [[usize; 6]; n]
    };
    let mut rs = 1;
    for aa in aaa {
        let sum = aa.into_iter().sum::<usize>();
        rs = rs * sum % (1e9 as usize + 7);
    }
    println!("{rs}");
}
