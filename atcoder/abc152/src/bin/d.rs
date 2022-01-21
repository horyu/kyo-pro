#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
    };
    let mut vv = [[0usize; 10]; 10];
    for i in 1..=n {
        let tail = i % 10;
        let head = {
            let mut h = i;
            while h >= 10 {
                h /= 10;
            }
            h
        };

        vv[head][tail] += 1;
    }

    let mut rs = 0;
    for t in 0..=9 {
        for h in 0..=9 {
            rs += vv[t][h] * vv[h][t];
        }
    }
    println!("{rs}");
}
