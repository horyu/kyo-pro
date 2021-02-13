#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        pp: [usize; n]
    };
    let mut min = pp[0];
    let mut rs = 0;
    for p in pp {
        if p <= min {
            rs += 1;
        }
        min = min.min(p);
    }
    println!("{}", rs);
}
