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

use ac_library::{LazySegtree, MapMonoid, Max};
struct MaxMonoid;
impl MapMonoid for MaxMonoid {
    type M = Max<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &Self::F, &x: &Self::F) -> Self::F {
        f + x
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

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
    let i2i = hsi
        .into_iter()
        .sorted_unstable()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();
    let j2j = hsj
        .into_iter()
        .sorted_unstable()
        .enumerate()
        .map(|(j, y)| (y, j))
        .collect::<HashMap<_, _>>();
    let mut bb = bb
        .into_iter()
        .map(|(x, y)| (i2i[&x], j2j[&y]))
        .collect_vec();
    let ww = ww
        .into_iter()
        .map(|(x, y)| (i2i[&x], j2j[&y]))
        .collect_vec();
    let h = i2i.len();
    let w = j2j.len();
    let mut lsi = ac_library::LazySegtree::<MaxMonoid>::new(h);
    let mut lsj = ac_library::LazySegtree::<MaxMonoid>::new(w);
    bb.sort_unstable_by_key(|&(i, j)| (R(j), R(i)));
    for (i, j) in bb.iter().copied() {
        let max = lsi.prod(i..);
        lsi.apply(i, max.max(j));
    }
    bb.sort_unstable_by_key(|&(i, j)| (R(i), R(j)));
    for (i, j) in bb.iter().copied() {
        let max = lsj.prod(j..);
        lsj.apply(j, max.max(i));
    }
    // for i in 0..h {
    //     let x = lsi.get(i);
    //     eprintln!("[{i}] {}", "#".repeat(x));
    // }
    // eprintln!();
    // for j in 0..w {
    //     let x = lsj.get(j);
    //     eprintln!("[{j}] {}", "#".repeat(x));
    // }
    for (i, j) in ww {
        if j <= lsi.prod(i..) || i <= lsj.prod(j..) {
            // 誤ったNo判定をしている
            println!("No");
            return;
        }
    }
    println!("Yes");
}
