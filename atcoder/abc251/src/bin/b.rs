#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        w: usize,
        aa: [usize; n]
    };
    let mut hs = HashSet::new();
    for i in 1..=3 {
        for vv in aa.iter().combinations(i) {
            let sum = vv.into_iter().sum::<usize>();
            hs.insert(sum);
        }
    }
    let rs = hs.into_iter().filter(|&x| x <= w).count();
    println!("{rs}");
}
