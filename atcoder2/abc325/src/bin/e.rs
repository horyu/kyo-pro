#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
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
