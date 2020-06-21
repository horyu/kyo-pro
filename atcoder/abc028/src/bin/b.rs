#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let mut vv = vec![0; 6];
    for c in s {
        vv[(c as usize) - (b'A' as usize)] += 1;
    }
    let s: String = vv
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", s);
}
