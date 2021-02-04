#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize
    };
    println!(
        "{} {}",
        a.saturating_sub(k),
        b.saturating_sub(k.saturating_sub(a))
    );
}
