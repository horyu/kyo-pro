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
        xxyycc: [(usize, usize, char); m],
    };
    let mut hsi = HashSet::from([0]);
    let mut hsj = HashSet::from([0]);
    let mut bb = vec![];
    let mut ww = vec![];
    for (x, y, c) in xxyycc.iter().copied() {
        hsi.insert(x);
        hsj.insert(y);
        if c == 'B' {
            bb.push((x, y));
        } else {
            ww.push((x, y));
        }
    }
    let x2i = hsi
        .into_iter()
        .sorted_unstable()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();
    let y2j = hsj
        .into_iter()
        .sorted_unstable()
        .enumerate()
        .map(|(j, y)| (y, j))
        .collect::<HashMap<_, _>>();
    let bb = bb
        .into_iter()
        .map(|(x, y)| (x2i[&x], y2j[&y]))
        .collect_vec();
    let ww = ww
        .into_iter()
        .map(|(x, y)| (x2i[&x], y2j[&y]))
        .collect_vec();
    let h = x2i.len();
    let w = y2j.len();
    let mut i2v = vec![0; h];
    let mut j2v = vec![0; w];
    for (i, j) in bb.iter().copied() {
        i2v[i] = i2v[i].max(j);
        j2v[j] = j2v[j].max(i);
    }
    let mut max = 0;
    for i in (0..h).rev() {
        max = max.max(i2v[i]);
        i2v[i] = max;
    }
    let mut max = 0;
    for j in (0..w).rev() {
        max = max.max(j2v[j]);
        j2v[j] = max;
    }
    for (i, j) in ww {
        if j <= i2v[i] || i <= j2v[j] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
