#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        tt: [isize; n],
        m: usize,
        ppxx: [(Usize1, isize); m]
    };
    let sum: isize = tt.iter().sum();
    for (p, x) in ppxx {
        println!("{}", sum - (tt[p] - x));
    }
}
