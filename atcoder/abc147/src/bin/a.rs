#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        aaa: [isize; 3]
    };
    println!(
        "{}",
        ["bust", "win"][(aaa.iter().sum::<isize>() <= 21) as usize]
    );
}
