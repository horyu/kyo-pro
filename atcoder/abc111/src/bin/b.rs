#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    println!(
        "{}",
        (n..)
            .find(|x| {
                let s = x.to_string();
                let first_char = s.chars().next().unwrap();
                s.chars().all(|c| c == first_char)
            })
            .unwrap()
    )
}
