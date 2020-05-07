#![allow(unused_imports)]
use proconio::{input, marker::Usize1};
use itertools::Itertools;
use ascii::{AsciiChar, AsciiStr};

fn main() {
    input!{
        n: usize,
        ww: [isize; n],
    };
    let mut min = isize::MAX;
    for i in 1..n {
        let left: isize = ww[0..i].iter().sum();
        let right: isize = ww[i..n].iter().sum();
        let diff = (left - right).abs();
        if diff < min {
            min = diff;
        }
    }
    println!("{}", min);

}
