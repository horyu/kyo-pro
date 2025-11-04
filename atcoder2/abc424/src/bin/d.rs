#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        t: usize,
    };
    // https://atcoder.jp/contests/abc424/editorial/13932
    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            ss: [Chars; h],
        };
        let k = 1usize << w;
        let mut allow = vec![vec![true; k]; k];
        for i in 0..k {
            for j in 0..k {
                for ii in 0..(w - 1) {
                    if (i >> ii) & 3 == 3 && (j >> ii) & 3 == 3 {
                        allow[i][j] = false;
                        break;
                    }
                }
            }
        }
        const INF: usize = 1e11 as usize;
        let mut dp = vec![INF; k];
        dp[0] = 0;
        for i in 0..h {
            let mut state = 0;
            for j in 0..w {
                if ss[i][j] == '#' {
                    state |= 1 << j;
                }
            }
            let mut dp2 = vec![INF; k];
            for j in 0..k {
                if j | state == state {
                    for jj in 0..k {
                        if allow[jj][j] {
                            dp2[j] = dp2[j].min(dp[jj] + (j ^ state).count_ones() as usize);
                        }
                    }
                }
            }
            dp = dp2;
        }
        let rs = dp.into_iter().min().unwrap();
        println!("{rs}");
    }
}
