#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        p: isize,
        k: isize,
        aaa: [[isize; n]; n],
    };
    let f = |kk| {
        let mut dist = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if aaa[i][j] == -1 {
                    dist[i][j] = kk;
                } else {
                    dist[i][j] = aaa[i][j];
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
        let mut c = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if dist[i][j] <= p {
                    c += 1;
                }
            }
        }
        c
    };
    let g = |num| {
        let mut l = 1;
        let mut r = 5_000_000_000;
        for _ in 0..40 {
            let mid = (l + r) / 2;
            if f(mid) <= num {
                r = mid;
            } else {
                l = mid;
            }
        }
        r
    };
    let ll = g(k);
    let rr = g(k - 1);
    if 2_000_000_000 <= rr - ll {
        println!("Infinity");
    } else {
        println!("{}", rr - ll);
    }
}
