#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut hh: [usize; n],
    };
    if n <= k {
        println!("0");
        return;
    }
    hh.sort_unstable();
    let rs = hh[..(n - k)].iter().sum::<usize>();
    println!("{}", rs);
}
