#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: Chars,
        b: Chars,
        c: Chars
    };
    println!(
        "{}",
        ["NO", "YES"][((a.last().unwrap() == b.first().unwrap())
            && (b.last().unwrap() == c.first().unwrap())) as usize]
    );
}
