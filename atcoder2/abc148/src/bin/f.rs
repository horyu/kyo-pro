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
        u: Usize1,
        v: Usize1,
        aabb: [(Usize1, Usize1); n - 1],
    };
    // https://at274.hatenablog.com/entry/2020/01/02/110121
    let mut g = vec![vec![]; n];
    for (a, b) in aabb {
        g[a].push(b);
        g[b].push(a);
    }
    // l = u-vの距離
    // l.div_ceil(2)
    let mut qq = VecDeque::new();
    let mut ddd = vec![vec![0; n]; 2];
    for i in [0, 1] {
        let start = [u, v][i];
        qq.push_back((start, 0, !0));
        while let Some((qi, qd, qf)) = qq.pop_front() {
            ddd[i][qi] = qd;
            for &j in &g[qi] {
                if j != qf {
                    qq.push_back((j, qd + 1, qi));
                }
            }
        }
    }
    // for dd in &ddd {
    //     eprintln!("{dd:?}");
    // }
    let rs = izip!(&ddd[0], &ddd[1])
        .map(|(&du, &dv)| if du < dv { dv } else { 0 })
        .max()
        .unwrap()
        - 1;
    println!("{rs}");
}
