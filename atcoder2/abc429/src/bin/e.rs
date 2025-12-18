#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uuvv: [(Usize1, Usize1); m],
        s: Chars,
    };
    // https://atcoder.jp/contests/abc429/editorial/14284
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    let ttff = s.iter().copied().map(|c| c == 'S').collect_vec();
    let mut ff = vec![[!0; 2]; n];
    let mut dd = ff.clone();
    let mut qq = VecDeque::new();
    for (i, tf) in ttff.iter().copied().enumerate() {
        if tf {
            ff[i][0] = i;
            dd[i][0] = 0;
            qq.push_back((i, i, 0));
        }
    }
    while let Some((base, i, dep)) = qq.pop_front() {
        for e in g[i].iter().copied() {
            if ff[e][0] == !0 {
                ff[e][0] = base;
                dd[e][0] = dep + 1;
                qq.push_back((base, e, dep + 1));
            } else if ff[e][1] == !0 && ff[e][0] != base {
                ff[e][1] = base;
                dd[e][1] = dep + 1;
                qq.push_back((base, e, dep + 1));
            }
        }
    }
    let rs = izip!(ttff, dd)
        .filter_map(|(tf, dd)| (!tf).then(|| dd[0] + dd[1]))
        .join("\n");
    println!("{rs}");
}
