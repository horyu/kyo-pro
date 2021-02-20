#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        pp: [usize; n],
    };
    // Pi番目の期待値は (Pi+1)/2
    // 最後に期待値にする
    let mut pre = pp[..k].iter().sum::<usize>();
    let mut max = pre;
    for i in 0..(n - k) {
        pre = pre + pp[k + i] - pp[i];
        max = max.max(pre);
    }
    println!("{}", (max + k) as f64 / 2.0);
}
