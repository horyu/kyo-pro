#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut abc: [u8; 3]
    };
    abc.sort();
    abc.dedup();
    println!("{}", abc.len());
}
