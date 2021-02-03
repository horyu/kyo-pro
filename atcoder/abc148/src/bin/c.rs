#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    println!("{}", lcm(a, b));
}

// http://www.nct9.ne.jp/m_hiroi/linux/rust01.html#chap06
fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
