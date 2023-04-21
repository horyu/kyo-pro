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
        aa: [usize; 2 * n],
    };
    let nn = 2 * n;
    let mut dp = vec![vec![!0usize >> 2; nn]; nn];
    for (i, (ax, ay)) in aa.iter().copied().tuple_windows().enumerate() {
        dp[i][i + 1] = ax.abs_diff(ay);
    }
    for size in (4..=nn).step_by(2) {
        for l in 0..=(nn - size) {
            let r = l + size - 1;
            dp[l][r] = ((l + 1)..(r - 1))
                .step_by(2)
                .map(|i| dp[l][i] + dp[i + 1][r])
                .chain([aa[l].abs_diff(aa[r]) + dp[l + 1][r - 1]])
                .min()
                .unwrap();
        }
    }
    // for dp in &dp {
    //     eprintln!("{}", dp.iter().map(|x| format!("{x:3}")).join(" "));
    // }
    let rs = dp[0][nn - 1];
    println!("{rs}");
}
