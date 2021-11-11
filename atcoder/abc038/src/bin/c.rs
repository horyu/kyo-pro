#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut cnts = vec![1; n];
    for (i, vv) in aa.windows(2).enumerate().rev() {
        if vv[0] < vv[1] {
            cnts[i] = cnts[i + 1] + 1;
        }
    }
    println!("{}", cnts.iter().sum::<usize>());
}
