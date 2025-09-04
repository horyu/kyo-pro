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
        s: Chars,
    };
    let bbcc = s
        .into_iter()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, it)| (c == '0', it.count()))
        .collect_vec();
    let len = bbcc.len();
    // 尺取法
    let mut rs = 0;
    let mut sum = 0;
    let mut l = 0;
    let mut ops_cnt = 0;
    for r in 0..len {
        let (b, c) = bbcc[r];
        sum += c;
        if b {
            ops_cnt += 1;
        }
        while k < ops_cnt {
            let (b, c) = bbcc[l];
            sum -= c;
            if b {
                ops_cnt -= 1;
            }
            l += 1;
        }
        rs = rs.max(sum);
    }
    println!("{rs}");
}
