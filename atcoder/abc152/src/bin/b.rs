#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    println!(
        "{}",
        if a < b {
            a.to_string().repeat(b)
        } else {
            b.to_string().repeat(a)
        }
    );
}
