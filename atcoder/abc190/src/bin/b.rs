#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        xxyy: [(usize, usize); n]
    };
    for (x, y) in xxyy {
        if (x < s) && (y > d) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
