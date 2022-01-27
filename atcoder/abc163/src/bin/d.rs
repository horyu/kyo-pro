#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    // n=4 k=2
    // 01234
    // 0+1, 0+2, 0+3, 0+4, 1+4, 2+4, 3+4
    // あるkに対して先頭k個の総和..=末尾k個の総和
    let mut rs = 0;
    const MOD: usize = 1e9 as usize + 7;
    for i in k..=(n + 1) {
        let l_sum = (i - 1) * i / 2;
        let r_sum = ((n + 1 - i) + n) * i / 2;
        rs = (rs + r_sum + 1 - l_sum) % MOD;
    }
    println!("{rs}");
}
