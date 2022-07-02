#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aaa: [Bytes; n]
    };
    let aaa = aaa
        .into_iter()
        .map(|aa| aa.into_iter().map(|a| (a - b'0') as usize).collect_vec())
        .collect_vec();
    let mut rs = 0;
    for i in 0..n {
        for j in 0..n {
            let mut vv = [aaa[i][j]; 8];
            for k in 1..n {
                for v in vv.iter_mut() {
                    *v *= 10;
                }
                vv[0] += aaa[(i + k) % n][j];
                vv[1] += aaa[(i + n - k) % n][j];
                vv[2] += aaa[i][(j + k) % n];
                vv[3] += aaa[i][(j + n - k) % n];
                vv[4] += aaa[(i + k) % n][(j + k) % n];
                vv[5] += aaa[(i + k) % n][(j + n - k) % n];
                vv[6] += aaa[(i + n - k) % n][(j + k) % n];
                vv[7] += aaa[(i + n - k) % n][(j + n - k) % n];
            }
            rs = rs.max(vv.into_iter().max().unwrap());
        }
    }
    println!("{rs}");
}
