#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        k: usize,
        x: usize,
    };
    println!("{}", ["No", "Yes"][(500 * k >= x) as usize]);
}
