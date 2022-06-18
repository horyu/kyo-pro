#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        hh: [isize; 3],
        ww: [isize; 3],
    };
    let mut rs = 0;
    // x0 x1 x2 :h0
    // x3 x4 x5 :h1
    // x6 x7 x8 :h2
    // w0 w1 w2
    // x0134 が定まればすべて定まる
    for x0 in 1..=28 {
        for x1 in 1..=28 {
            let x2 = hh[0] - x0 - x1;
            if x2 < 1 {
                break;
            }
            for x3 in 1..=28 {
                let x6 = ww[0] - x0 - x3;
                if x6 < 1 {
                    break;
                }
                for x4 in 1..=28 {
                    let x5 = hh[1] - x3 - x4;
                    if x5 < 1 {
                        break;
                    }
                    let x7 = ww[1] - x1 - x4;
                    if x7 < 1 {
                        break;
                    }
                    let x8h = hh[2] - x6 - x7;
                    let x8w = ww[2] - x2 - x5;
                    if x8h == x8w && x8h > 0 {
                        rs += 1;
                    }
                }
            }
        }
    }
    println!("{rs}");
}