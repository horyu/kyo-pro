#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        c: isize,
        bb: [isize; m],
        aaa: [[isize; m]; n]
    };
    let rs = aaa
        .iter()
        .filter(|&aa| 0 < (0usize..m).fold(c, |acc, i| acc + aa[i] * bb[i]))
        .count();
    println!("{}", rs);
}
