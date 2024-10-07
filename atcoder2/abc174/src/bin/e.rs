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
        k: usize,
        aa: [usize; n],
    };
    // 最大値の最小化
    let mut ng = 0usize;
    let mut ok = 1e9 as usize;
    while 1 < ok - ng {
        let m = (ok + ng) / 2;
        let mut tmp = 0;
        for &a in &aa {
            tmp += a.div_ceil(m) - 1;
        }
        if tmp <= k {
            ok = m;
        } else {
            ng = m;
        }
    }
    let rs = ok;
    println!("{rs}");
}
