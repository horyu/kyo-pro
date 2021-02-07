#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        v: usize,
        t: usize,
        s: usize,
        d: usize,
    };
    println!(
        "{}",
        ["Yes", "No"][((v * t)..=(v * s)).contains(&d) as usize]
    );
}
