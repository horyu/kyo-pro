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
        q: usize,
        pp: [Usize1; n - 1],
    };
    // https://atcoder.jp/contests/arc148/editorial/4775
    let mut cc = vec![0; n];
    for (i, p) in pp.iter().copied().enumerate() {
        cc[p] += 1;
    }
    let pp = chain!([!0], pp).collect_vec();
    for _ in 0..q {
        input! {
            m: usize,
            vv: [Usize1; m],
        };
        let mut rs = 0;
        let hs = vv.iter().copied().collect::<HashSet<_>>();
        for v in vv.iter().copied() {
            rs += cc[v];
            if hs.contains(&pp[v]) {
                rs -= 1;
            } else {
                rs += 1;
            }
        }
        println!("{rs}");
    }
}
