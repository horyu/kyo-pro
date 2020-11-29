#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String
    };
    let num = s
        .as_str()
        .split(|c: char| !matches!(c, 'A' | 'C' | 'G' | 'T'))
        .map(|x| x.len())
        .max()
        .unwrap();
    println!("{}", num);
}
