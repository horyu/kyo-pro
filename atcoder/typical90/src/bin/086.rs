#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        q: usize,
        xxyyzzww: [(usize, usize, usize, usize); q],
    };

    let mut rs = 1usize;
    for i in 0..60 {
        let mut ww = [0; 51];
        for (j, xyzw) in xxyyzzww.iter().enumerate() {
            ww[j + 1] = (xyzw.3 / (1 << i)) % 2;
        }
        let mut tmp = 0;
        let mut bit = [0; 13];
        for j in 0..(1 << n) {
            for k in 0..n {
                bit[k + 1] = (j / (1 << k)) % 2;
            }

            if xxyyzzww
                .iter()
                .enumerate()
                .all(|(k, &(x, y, z, _))| bit[x] | bit[y] | bit[z] == ww[k + 1])
            {
                tmp += 1;
            }
        }
        rs *= tmp;
        rs %= MOD;
    }

    println!("{rs}");
}
