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
        q: usize,
        xxyy: [(Usize1, Usize1); m],
        aabb: [(Usize1, Usize1); q],
    };
    let mut g = vec![vec![]; n];
    for (x, y) in xxyy {
        g[x].push(y);
    }
    let mut bb = (0..n)
        .map(|i| vec![0u128; n.div_ceil(128) - i / 128])
        .collect_vec();
    let mut checked = vec![false; n];
    for i in 0..n {
        dfs(&g, &mut bb, &mut checked, i);
    }

    for (a, b) in aabb {
        let tf = bb[a][b / 128 - a / 128] & (1 << (b % 128)) != 0;
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}

fn dfs(g: &Vec<Vec<usize>>, bb: &mut Vec<Vec<u128>>, checked: &mut Vec<bool>, x: usize) {
    if checked[x] {
        return;
    }
    bb[x][0] |= 1 << (x % 128);
    for &y in &g[x] {
        dfs(g, bb, checked, y);
        let mut tmp = std::mem::take(&mut bb[x]);
        for (t, yb) in izip!(tmp.iter_mut().rev(), bb[y].iter().copied().rev()) {
            *t |= yb;
        }
        bb[x] = tmp;
    }
    checked[x] = true;
}
