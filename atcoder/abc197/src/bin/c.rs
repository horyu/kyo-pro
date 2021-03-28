#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [i32; n]
    };
    let mut min = aa.iter().fold(0, |acc, &a| acc ^ a);
    for i in 2..n {
        // i 個のグループに分ける
        for indexes in (1..n).combinations(i - 1) {
            let xor = std::iter::once(0usize)
                .chain(indexes.into_iter())
                .chain(std::iter::once(n))
                .tuple_windows()
                .map(|(mae, ato)| aa[mae..ato].iter().fold(0, |acc, &a| acc | a))
                .fold(0, |acc, a| acc ^ a);
            min = min.min(xor);
        }
    }
    println!("{}", min);
}
