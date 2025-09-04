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
        m: usize,
        uuvvcc: [(Usize1, Usize1, usize); m],
    };
    let mut hm = HashMap::new();
    for &(u, v, c) in &uuvvcc {
        hm.entry((u, v)).or_insert(c);
        hm.entry((v, u)).or_insert(c);
    }
    let aabbcc = pathfinding::undirected::kruskal::kruskal_indices(n, &uuvvcc).collect_vec();
    let mut g = vec![vec![]; n];
    for &(u, v, _c) in &aabbcc {
        g[u].push(v);
        g[v].push(u);
    }
    let mut rs = vec![0; n];
    let mut qq = VecDeque::new();
    qq.push_back((!0, 0));
    rs[0] = 1;
    while let Some((qfrom, qi)) = qq.pop_front() {
        for &i in &g[qi] {
            if i == qfrom {
                continue;
            }
            let c = hm.get(&(qi, i)).copied().unwrap();
            if rs[qi] == c {
                rs[i] = (rs[qi] % n) + 1;
            } else {
                rs[i] = c;
            }
            qq.push_back((qi, i));
        }
    }
    let rs = rs.into_iter().join("\n");
    println!("{}", rs);
}
