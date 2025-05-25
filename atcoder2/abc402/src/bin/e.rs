#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        x: usize,
        ssccpp: [(f64, usize, f64); n],
    };
    // BitDP
    // dp[mask][j] := maskの問題が解く対象であるとき、j円払った場合の期待値
    let mut dp = vec![vec![0.0; x + 1]; 1usize << n];
    for bits in 1..(1usize << n) {
        if bits.count_ones() == 1 {
            let i = bits.trailing_zeros() as usize;
            let (s, c, p) = ssccpp[i];
            let p = p / 100.0;
            let q = 1.0 - p;
            for j in c..=x {
                dp[bits][j] = dp[bits][j - c] + s * (p * q.powi((j / c - 1) as i32));
            }
            if x <= 5 {
                eprintln!("{i} {:?}", dp[bits]);
            }
            continue;
        }
        let ii = (0..n)
            .filter(|&i| 0 != (bits & (1usize << i)))
            .collect_vec();
        for j in 1..=x {
            let mut max = 0.0f64;
            for i in ii.iter().copied() {
                let (s, c, p) = ssccpp[i];
                if j < c {
                    continue;
                }
                let p = p / 100.0;
                let q = 1.0 - p;
                // let v = (dp[bits ^ (1usize << i)][j - c] + s) * p + s * (p * q.powi((j / c - 1) as i32));
                let v = dp[bits ^ (1usize << i)][j - c] + s * p
                     + dp[bits][j - c] * (1.0 - p) * q.powi((j / c - 1) as i32);
                max = max.max(v);
            }
            dp[bits][j] = max;
        }
    }
    if x <= 5 {
        for (bits, dp) in dp.iter().enumerate() {
            eprintln!("{:05b} {:?}", bits, dp);
        }
    }
    // if x <= 5 {
    //     eprintln!("@{:?}", dp[(1usize << n) - 1]);
    // }
    let rs = dp[(1usize << n) - 1]
        .iter()
        .fold(0.0f64, |acc, v| acc.max(*v));
    println!("{rs}");
}
