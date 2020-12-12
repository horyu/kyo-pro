#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        x: usize,
        ll: [usize; n]
    };
    let mut count = 1;
    let mut pos = 0;
    for l in ll {
        pos += l;
        if pos <= x {
            count += 1;
            continue;
        }
        break;
    }
    println!("{}", count);
}
