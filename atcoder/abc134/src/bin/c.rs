#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut bb = aa.clone();
    bb.sort_unstable();
    bb.reverse();
    let x = bb[0];
    let y = bb[1];
    for a in aa {
        println!("{}", [x, y][(a == x) as usize]);
    }
}
