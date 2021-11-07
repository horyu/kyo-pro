#![allow(unused_imports)]
#![allow(clippy::many_single_char_names)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: u128,
        k: u128,
    };
    let mut cnt = 0u128;
    // === *1
    cnt += 1;
    // <== *3
    cnt += (k - 1) * 3;
    // <=< *6
    cnt += (k - 1) * (n - k) * 6;
    // ==< *3
    cnt += (n - k) * 3;

    println!("{}", cnt as f64 / n.pow(3) as f64);
}
