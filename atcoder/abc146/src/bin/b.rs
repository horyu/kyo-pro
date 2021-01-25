#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: u8,
        s: Chars
    };
    let rs: String = s
        .into_iter()
        .map(|c| (((c as u8) + n - b'A') % 26 + b'A') as char)
        .collect();
    println!("{}", rs);
}
