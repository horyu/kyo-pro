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
        w: usize,
        xxyy: [(Usize1, Usize1); n],
        q: usize,
        ttaa: [(usize, Usize1); q],
    };
    // dd[i] = 下からi番目の層の最大値
    let mut dd = vec![0];
    // cc[i] = i列目のブロックの個数
    let mut cc = vec![0; w];
    // i2d[i] = ブロックiの層
    let mut i2d = vec![0; n];
    for (i, (x, y)) in xxyy
        .iter()
        .copied()
        .enumerate()
        .sorted_unstable_by_key(|ixy| ixy.1 .1)
    {
        if let Some(d) = dd.get_mut(cc[x]) {
            *d = y.max(*d);
        } else {
            dd.push(y);
        }
        i2d[i] = cc[x];
        cc[x] += 1;
    }
    let c_min = cc.iter().copied().min().unwrap();
    for (t, a) in ttaa {
        let d = i2d[a];
        let tf = c_min <= d || t <= dd[d];
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}
