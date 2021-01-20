#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    if (n >= 2) && (n % 2 == 0) && (s[..n / 2] == s[n / 2..]) {
        println!("Yes");
        return;
    }
    println!("No");
}
