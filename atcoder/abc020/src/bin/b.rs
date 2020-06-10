#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut a: Chars,
        mut b: Chars,
    };
    a.append(&mut b);
    let s: String = a.into_iter().collect();
    let i: usize = s.parse().unwrap();
    println!("{}", i * 2);
}
