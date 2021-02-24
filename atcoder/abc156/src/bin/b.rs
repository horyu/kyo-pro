#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut n: usize,
        k: usize,
    };
    let mut num = 0;
    while n > 0 {
        n = n / k;
        num += 1;
    }
    println!("{}", num);
}
