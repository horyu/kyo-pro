#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        k: usize,
        xx: [Usize1; n],
        aa: [usize; n],
    };
    // ダブリング　https://atcoder.jp/contests/abc367/editorial/10707
    let mut pp = vec![vec![0; n]; 60];
    pp[0] = xx;
    for d in 1..60 {
        for i in 0..n {
            pp[d][i] = pp[d - 1][pp[d - 1][i]];
        }
    }
    let mut qq = (0..n).collect_vec();
    for d in 0..60 {
        if (k >> d) & 1 != 0 {
            qq = qq.iter().map(|&i| pp[d][i]).collect_vec();
        }
    }
    let rs = qq.into_iter().map(|q| aa[q]).join(" ");
    println!("{rs}");
}
