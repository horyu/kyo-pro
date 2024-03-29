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
        m: usize,
    };
    let i2rr = (0..m)
        .map(|_| {
            input! {
                k: usize,
                rr: [Usize1; k],
            };
            rr
        })
        .collect_vec();
    let mut r2ii = vec![vec![]; n];
    for (i, rr) in i2rr.iter().enumerate() {
        for &r in rr {
            r2ii[r].push(i);
        }
    }

    let mut qq = VecDeque::new();
    let mut checked_i = vec![false; m];

    let mut rs = vec![-1; n];
    rs[0] = 0;
    qq.push_back(0);

    while let Some(qr) = qq.pop_front() {
        for &i in &r2ii[qr] {
            if !checked_i[i] {
                checked_i[i] = true;
                for &r in &i2rr[i] {
                    if rs[r] == -1 {
                        rs[r] = rs[qr] + 1;
                        qq.push_back(r);
                    }
                }
            }
        }
    }

    let rs = rs.iter().join("\n");
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
    };
    let mut g = vec![vec![]; n + m];
    for j in 0..m {
        input! {
            k: usize,
            rr: [Usize1; k],
        };
        for r in rr.iter().copied() {
            g[r].push((n + j, 1usize));
            g[n + j].push((r, 1));
        }
    }
    let hm = pathfinding::directed::dijkstra::dijkstra_all(&0, |&i| g[i].iter().copied());
    println!("0");
    for i in 1..n {
        if let Some((_, d)) = hm.get(&i) {
            println!("{}", d / 2);
        } else {
            println!("-1");
        }
    }
}
