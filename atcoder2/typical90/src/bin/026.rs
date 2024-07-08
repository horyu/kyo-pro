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
        aabb: [(Usize1, Usize1); n -1],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in aabb.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    let mut xxx = vec![vec![]; 2];
    let mut qq = VecDeque::new();
    qq.push_back((0, 0, !0));
    while let Some((qi, qd, qf)) = qq.pop_front() {
        xxx[qd % 2].push(qi);
        if xxx[qd % 2].len() == n / 2 {
            let rs = std::mem::take(&mut xxx[qd % 2])
                .into_iter()
                .map(|x| x + 1)
                .sorted_unstable()
                .join(" ");
            println!("{rs}");
            return;
        }
        for i in g[qi].iter().copied() {
            if i == qf {
                continue;
            }
            qq.push_back((i, qd + 1, qi));
        }
    }
}
