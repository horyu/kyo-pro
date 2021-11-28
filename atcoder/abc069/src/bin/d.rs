#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        aa: [usize; n],
    };
    let mut v = Vec::with_capacity(h * w);
    for (i, a) in aa.into_iter().enumerate() {
        for _ in 0..a {
            v.push(i + 1);
        }
    }

    for y in 0..h {
        let range = &v[(w * y)..(w * (y + 1))];
        if y % 2 == 0 {
            println!("{}", range.iter().join(" "));
        } else {
            println!("{}", range.iter().rev().join(" "));
        }
    }
}
