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
        aabb: [(Usize1, Usize1); n]
    };
    const M: usize = 5000;
    // いもす法
    let k = k + 1;
    let mut vvv = vec![vec![0; M + k]; M + k];
    for (a, b) in aabb {
        vvv[a][b] += 1;
        vvv[a + k][b] -= 1;
        vvv[a][b + k] -= 1;
        vvv[a + k][b + k] += 1;
    }

    for i in 0..=M {
        for j in 0..M {
            vvv[i][j + 1] += vvv[i][j];
        }
    }
    for i in 0..M {
        for j in 0..=M {
            vvv[i + 1][j] += vvv[i][j];
        }
    }
    let mut rs = 0;
    for i in 0..=M {
        for j in 0..=M {
            rs = rs.max(vvv[i][j]);
        }
    }
    println!("{rs}");
}
