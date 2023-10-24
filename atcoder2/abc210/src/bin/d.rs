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
        h: usize,
        w: usize,
        c: isize,
        mut aaa: [[isize; w]; h],
    };
    // https://kanpurin.hatenablog.com/entry/2021/07/17/232213
    let solve = |aaa: &[Vec<isize>]| -> isize {
        let mut dp = vec![vec![isize::MAX; w + 1]; h + 1];
        let mut ans = 1 << 60;
        for i in 0..h {
            for j in 0..w {
                dp[i + 1][j + 1] = dp[i + 1][j].min(dp[i][j + 1]);
                if dp[i + 1][j + 1] != isize::MAX {
                    ans = ans.min(dp[i + 1][j + 1] + aaa[i][j] + c * (i + j) as isize);
                }
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(aaa[i][j] - c * (i + j) as isize);
            }
        }
        ans
    };
    let mut rs = isize::MAX;
    for i in 0..4 {
        if 0 < i & 1 {
            aaa.reverse();
        }
        if 0 < i & 2 {
            for aa in aaa.iter_mut() {
                aa.reverse();
            }
        }
        rs = rs.min(solve(&aaa));
    }
    println!("{rs}");
}
