#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    let aa = aa
        .iter()
        .enumerate()
        .map(|(i, a)| a - (i + 1) as isize)
        .sorted()
        .collect_vec();
    let b = if n % 2 == 1 {
        aa[n / 2]
    } else {
        (aa[n / 2 - 1] + aa[n / 2]) / 2
    };
    println!("{}", aa.iter().map(|a| (a - b).abs()).sum::<isize>());
}
