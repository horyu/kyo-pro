#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        aa: [usize; n],
    };
    // 半分全列挙
    let mut xxx = vec![vec![]; k + 1];
    let mut yyy = vec![vec![]; k + 1];
    xxx[0].push(0);
    yyy[0].push(0);

    for bit in 1usize..=(1 << (n / 2)) {
        let ones = bit.count_ones() as usize;
        if ones <= k {
            let mut sum = 0;
            for i in 0..(n / 2) {
                if 0 < bit & (1 << i) {
                    sum += aa[i];
                }
            }
            if 0 < sum {
                xxx[ones].push(sum);
            }
        }
    }
    for bit in 1usize..=(1 << (n - n / 2)) {
        let ones = bit.count_ones() as usize;
        if ones <= k {
            let mut sum = 0;
            for i in 0..(n - n / 2) {
                if 0 < bit & (1 << i) {
                    sum += aa[n / 2 + i];
                }
            }
            if 0 < sum {
                yyy[ones].push(sum);
            }
        }
    }

    for zz in xxx.iter_mut().chain(yyy.iter_mut()) {
        zz.sort_unstable();
    }

    let mut rs = 0usize;
    for i in 0..=k {
        let j = k - i;
        for &x in &xxx[i] {
            rs += yyy[j].partition_point(|&y| x + y <= p);
        }
    }
    println!("{rs}");
}
