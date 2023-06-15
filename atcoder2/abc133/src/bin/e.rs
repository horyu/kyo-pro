#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        aabb: [(Usize1, Usize1); n - 1]
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb {
        g[a].push(b);
        g[b].push(a);
    }
    let mut qq = VecDeque::new();
    qq.push_back((!0, 0, 0, 0));

    let mut rs = ModInt1000000007::new(1);
    while let Some((qf, qt, qj, qd)) = qq.pop_front() {
        // eprintln!("{qf}-{qt}-{qj}-{qd}: {}", k.saturating_sub(qj + qd.min(2)));
        rs *= k.saturating_sub(qj + qd.min(2));
        let mut j = 0;
        for &t in &g[qt] {
            if t != qf {
                qq.push_back((qt, t, j, qd + 1));
                j += 1;
            }
        }
    }
    println!("{rs}");
}
