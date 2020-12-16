#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let rs = (b - a + 1) - (b / c - (a - 1) / c) - (b / d - (a - 1) / d)
        + (b / lcm(c, d) - (a - 1) / lcm(c, d));
    println!("{}", rs);
}

// http://www.nct9.ne.jp/m_hiroi/linux/rust01.html
// 最小公倍数
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
