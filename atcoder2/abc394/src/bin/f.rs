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
    let mut rs = -1;
    let mut memo = HashMap::new();
    for i in 0..n {
        if g[i].len() < 4 {
            continue;
        }
        rs = rs.max(dfs(&g, &mut memo, !0, i));
    }
    println!("{rs}");
}

fn dfs(
    g: &[Vec<usize>],
    memo: &mut HashMap<(usize, usize), isize>,
    par: usize,
    cur: usize,
) -> isize {
    if let Some(&v) = memo.get(&(par, cur)) {
        return v;
    }
    if g[cur].len() < 4 {
        memo.insert((par, cur), 1);
        return 1;
    }
    let mut ss = vec![];
    for next in g[cur].iter().copied() {
        if next == par {
            continue;
        }
        ss.push(dfs(g, memo, cur, next));
    }
    ss.sort_unstable();
    let rs = 1 + ss
        .iter()
        .rev()
        .take(3 + usize::from(par == !0))
        .sum::<isize>();
    memo.insert((par, cur), rs);
    rs
}
