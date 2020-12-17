#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        pp: [usize; n]
    };
    let rs = pp
        .windows(3)
        .filter(|ppp| {
            ((ppp[0] < ppp[1]) && (ppp[1] < ppp[2])) || ((ppp[0] > ppp[1]) && (ppp[1] > ppp[2]))
        })
        .count();
    println!("{}", rs);
}
