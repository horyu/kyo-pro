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
        pp: [usize; n - 1],
        xxyy: [(Usize1, usize); m],
    };
    let mut ccc = vec![vec![]; n];
    for (i, p) in pp.iter().copied().enumerate() {
        ccc[p - 1].push(i + 1);
    }
    // for (i, cc) in ccc.iter().enumerate() {
    //     eprintln!("{i} {cc:?}");
    // }
    let mut hm = HashMap::new();
    for (x, y) in xxyy {
        let e = hm.entry(x).or_insert(0);
        *e = (*e).max(y + 1);
    }
    let mut hs = HashSet::new();
    dfs(
        &mut hs,
        &hm,
        0,
        hm.get(&0).copied().unwrap_or_default(),
        &ccc,
    );
    let rs = hs.len();
    // dbg!(hs);
    println!("{rs}");
}

fn dfs(
    hs: &mut HashSet<usize>,
    hm: &HashMap<usize, usize>,
    i: usize,
    y: usize,
    ccc: &[Vec<usize>],
) {
    if 0 < y {
        hs.insert(i);
    }
    for c in ccc[i].iter().copied() {
        let next_y = (y.saturating_sub(1)).max(hm.get(&c).copied().unwrap_or_default());
        dfs(hs, hm, c, next_y, ccc)
    }
}
