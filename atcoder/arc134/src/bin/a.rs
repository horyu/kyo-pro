#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        l: usize,
        w: usize,
        aa: [usize; n]
    };
    let mut rs = 0;
    let mut pre_r = 0;
    for x in aa.into_iter().chain(std::iter::once(l)) {
        if x > pre_r {
            rs += (x - pre_r + w - 1) / w;
        }
        pre_r = x + w;
    }
    println!("{rs}");
}
