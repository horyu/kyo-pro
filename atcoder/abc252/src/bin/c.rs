#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        ss: [Chars; n]
    };
    let mut ccc = vec![vec![0; 10]; 10];
    for s in ss {
        for (j, c) in s.into_iter().enumerate() {
            ccc[(c as u8 - b'0') as usize][j] += 1;
        }
    }
    let mut rs = 10000;
    for mut cc in ccc {
        let mut sum = 0;
        for t in 0.. {
            let tt = t % 10;
            if cc[tt] > 0 {
                cc[tt] -= 1;
                sum += 1;
                if sum == n {
                    rs = rs.min(t);
                    break;
                }
            }
        }
    }
    println!("{rs}");
}
