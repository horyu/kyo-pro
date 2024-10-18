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
        aabb: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }

    let bfs = |start: usize| -> Vec<usize> {
        let mut dd = vec![!0; n];
        let mut qq = VecDeque::new();
        qq.push_back(start);
        dd[start] = 0;
        while let Some(v) = qq.pop_front() {
            for w in g[v].iter().copied() {
                if dd[w] == !0 {
                    dd[w] = dd[v] + 1;
                    qq.push_back(w);
                }
            }
        }
        dd
    };

    let xx = bfs(0);
    let yy = bfs(n - 1);
    let mut cnt = 0;
    for (x, y) in izip!(xx, yy) {
        if x <= y {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }
    let rs = ["Snuke", "Fennec"][(0 < cnt) as usize];
    println!("{rs}");
}
