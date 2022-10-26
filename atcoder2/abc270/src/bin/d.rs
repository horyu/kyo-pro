#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; k],
    };
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        for &a in &aa {
            if i < a {
                break;
            }
            dp[i] = dp[i].max(i.saturating_sub(dp[i - a]));
        }
    }
    let rs = dp[n];
    println!("{rs}");
}
