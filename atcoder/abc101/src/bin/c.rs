#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        _aa: [usize; n]
    };
    if n == k {
        println!("1");
        return;
    }
    let mut min = 100000;
    // 先頭 i から k-1 ずつ範囲を拡大していく
    // iを 2 ~ k までずらしていく
    for i in 2..=k {
        let nokori = n - i;
        let count = 1 + nokori / (k - 1) + ((nokori % (k - 1) != 0) as usize);
        min = std::cmp::min(min, count);
    }
    println!("{}", min);
}
