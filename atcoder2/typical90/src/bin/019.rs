#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [isize; 2 * n],
    };
    let nn = 2 * n;
    let mut dp = vec![vec![1e9 as isize; nn + 1]; nn + 1];
    for i in 0..(nn - 1) {
        dp[i][i + 2] = (aa[i] - aa[i + 1]).abs();
    }
    for range in (4..=nn).step_by(2) {
        for l in 0..=(nn - range) {
            let r = l + range;
            // eprintln!("{range} {l}-{r}");
            dp[l][r] = (aa[l] - aa[r - 1]).abs() + dp[l + 1][r - 1];
            for m in ((l + 2)..=(r - 2)).step_by(2) {
                dp[l][r] = dp[l][r].min(dp[l][m] + dp[m][r]);
            }
            // dp[l][r] = [
            //     (aa[l] - aa[r - 1]).abs() + dp[l + 1][r - 1],
            //     dp[l][l + 2] + dp[l + 2][r],
            //     dp[l][r - 2] + dp[r - 2][r],
            // ]
            // .iter()
            // .min()
            // .copied()
            // .unwrap();
        }
    }
    let rs = dp[0][nn];
    // for (i, dp) in dp.into_iter().enumerate() {
    //     let s = dp.into_iter().map(|x| if x == (1e9 as isize) { "___".to_string() } else { format!("{x:3}")}).join(",");
    //     eprintln!("[{i:2}] {}", s);
    // }
    println!("{rs}");
}
