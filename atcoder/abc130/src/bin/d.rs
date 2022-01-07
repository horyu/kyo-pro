#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };
    // 尺取り法
    let mut r = 0;
    let mut sum = 0;
    let mut rs = 0;
    for l in 0..n {
        while sum < k {
            if r == n {
                break;
            }
            sum += aa[r];
            r += 1;
        }
        if sum < k {
            break;
        }
        rs += n - r + 1;
        sum -= aa[l];
    }
    println!("{}", rs);
}
