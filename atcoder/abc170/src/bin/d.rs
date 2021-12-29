#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.sort_unstable();
    let max = aa[n - 1];
    let mut dp = vec![true; max + 1];
    let mut dup = 0;
    let mut i = 0;
    while i < aa.len() {
        let a = aa[i];
        if !dp[a] {
            i += 1;
            continue;
        }
        if i > 0 {
            let left = i - 1;
            if aa[left] == aa[i] {
                while aa[left] == aa[i] {
                    i += 1;
                    if i >= aa.len() {
                        break;
                    }
                }
                dup += i - left;
                continue;
            }
        }
        let mut na = 2 * a;
        while na <= max {
            dp[na] = false;
            na += a;
        }
        i += 1;
    }
    println!(
        "{}",
        aa.iter().filter(|&&a| dp[a]).collect::<Vec<_>>().len() - dup
    )
}
