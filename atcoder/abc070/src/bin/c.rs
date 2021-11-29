#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut tt: [usize; n]
    };
    tt.sort_unstable();
    tt.dedup();
    if tt.len() == 1 {
        println!("{}", tt[0]);
        return;
    }
    let ans = tt.iter().fold(1usize, |acc, &t| lcm(acc, t));
    println!("{}", ans);
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(m: usize, n: usize) -> usize {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}
