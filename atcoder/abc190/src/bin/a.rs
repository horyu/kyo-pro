#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    println!("{}", ["Takahashi", "Aoki"][(a + c <= b) as usize]);
}
