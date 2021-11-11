#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [u128; n]
    };
    // 愚直だが普通に通る
    // let rs = aa.windows(k).map(|v| v.iter().sum::<u128>()).sum::<u128>();
    let rs = aa.iter().sum::<u128>() * (k as u128)
        - (0..(k - 1))
            .map(|i| (k - 1 - i) as u128 * (aa[i] + aa[n - 1 - i]))
            .sum::<u128>();
    println!("{}", rs);
}
