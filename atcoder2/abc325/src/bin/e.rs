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
        a: usize,
        b: usize,
        c: usize,
        ddd: [[usize; n]; n],
    };
    let mut g = vec![vec![]; n * 2];
    for i in 0..n {
        g[i].push((n + i, 0));
    }
    for i in 0..n {
        for j in (i + 1)..n {
            let d = ddd[i][j];
            g[i].push((j, d * a));
            g[j].push((i, d * a));
            g[n + i].push((n + j, d * b + c));
            g[n + j].push((n + i, d * b + c));
        }
    }
    if let Some((_, rs)) = pathfinding::directed::dijkstra::dijkstra(
        &0,
        |&i| g[i].iter().copied(),
        |&i| i == 2 * n - 1,
    ) {
        println!("{rs}");
    } else {
        println!("-1");
    }
}

fn main2() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        ddd: [[usize; n]; n],
    };
    use std::cmp::Reverse as R;
    let mut bh = BinaryHeap::new();
    let mut checked = vec![vec![false; 2]; n];
    bh.push(R((0usize, 0, false)));
    while let Some(R((now, from, norikae))) = bh.pop() {
        if from == n - 1 {
            println!("{now}");
            return;
        }
        if !norikae && !checked[from][0] {
            checked[from][0] = true;
            for to in 1..n {
                if checked[to][0] {
                    continue;
                }
                bh.push(R((now + ddd[from][to] * a, to, false)));
            }
        }
        if checked[from][1] {
            continue;
        }
        checked[from][1] = true;
        for to in 1..n {
            if checked[to][1] {
                continue;
            }
            bh.push(R((now + ddd[from][to] * b + c, to, true)));
        }
    }
}
