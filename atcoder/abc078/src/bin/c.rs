#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: i64,
        m: i64
    };
    // k 番目までの期待値
    // Sum[i=1..k] (1 / (2^m)^i) * { 100*(n-m) + 1900*m }
    println!(
        "{}",
        (100 * (n - m) + 1900 * m) * 2i64.pow(m as u32)
    );
}
