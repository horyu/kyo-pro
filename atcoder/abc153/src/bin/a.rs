#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        a: usize,
    };
    println!("{}", h / a + ((h % a) != 0) as usize);
}
