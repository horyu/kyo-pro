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
        x: usize,
        aappbbqq: [(usize, usize, usize, usize); n],
    };
    // https://atcoder.jp/contests/abc374/editorial/11094
    // 最小値の最大値は二分探索
    let mut ok = 0;
    let mut ng = 1e10 as usize;
    while 1usize < ng - ok {
        let mid = (ok + ng) / 2;
        let mut cost = 0;
        for (a, p, b, q) in aappbbqq.iter().copied() {
            let mut tmp = 1e10 as usize;
            for cnt in 0..=b {
                let cur = cnt * p + mid.saturating_sub(a * cnt).div_ceil(b) * q;
                tmp = tmp.min(cur);
            }
            for cnt in 0..=a {
                let cur = cnt * q + mid.saturating_sub(b * cnt).div_ceil(a) * p;
                tmp = tmp.min(cur);
            }
            cost += tmp;
            if x < cost {
                break;
            }
        }
        if x < cost {
            ng = mid;
        } else {
            ok = mid;
        }
    }
    let rs = ok;
    println!("{rs}");
}
