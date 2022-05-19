#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        aa: [usize; n]
    };
    let mut ok = 1;
    let mut ng = l / k + 1;
    let f = |x: usize| -> bool {
        let mut cnt = 0;
        let mut left = 0;
        for &a in &aa {
            if a - left >= x && l - a >= x {
                cnt += 1;
                left = a;
            }
        }
        cnt >= k
    };
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
