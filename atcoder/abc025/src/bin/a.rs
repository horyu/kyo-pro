#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
        n: Usize1
    };
    println!("{}{}", s[n / 5], s[n % 5]);
}
