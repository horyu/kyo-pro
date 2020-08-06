#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        l: isize,
        h: isize,
        n: usize,
        aa: [isize; n]
    };
    for a in aa {
        println!(
            "{}",
            if a < l {
                l - a
            } else if a > h {
                -1
            } else {
                0
            }
        );
    }
}
