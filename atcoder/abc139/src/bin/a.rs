#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let rs = s.iter().zip(t.iter()).filter(|(a, b)| a == b).count();
    println!("{}", rs);
}
