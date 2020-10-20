#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize
    };
    println!("{}", [":(", "Yay!"][((a <= 8) && (b <= 8)) as usize]);
}
