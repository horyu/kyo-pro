#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const MAX_SIZE: usize = 100000usize.div_ceil(128);
const HALF_SIZE: usize = MAX_SIZE / 2;
const MAX_N: usize = MAX_SIZE * 128;
const HALF_N: usize = MAX_N / 2;

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
    // **
    // _*
    let mut bb = vec![vec![0u128; MAX_SIZE]; n.min(HALF_N)];
    if HALF_N < n {
        bb.extend(vec![vec![0u128; HALF_SIZE]; HALF_N]);
    }
    let mut checked = vec![false; n];
    for i in 0..n {
        dfs(&g, &mut bb, &mut checked, i);
    }

    for (a, b) in aabb {
        let tf = if a < HALF_N {
            bb[a][b / 128] & (1 << (b % 128)) != 0
        } else {
            bb[a][(b / 128) - HALF_SIZE] & (1 << (b % 128)) != 0
        };
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}

fn dfs(g: &Vec<Vec<usize>>, bb: &mut Vec<Vec<u128>>, checked: &mut Vec<bool>, x: usize) {
    if checked[x] {
        return;
    }
    if x < HALF_N {
        bb[x][x / 128] |= 1 << (x % 128);
    } else {
        bb[x][(x / 128) - HALF_SIZE] |= 1 << (x % 128);
    }
    for &y in &g[x] {
        dfs(g, bb, checked, y);
        let mut tmp = std::mem::take(&mut bb[x]);
        match (x < HALF_N, y < HALF_N) {
            (true, false) => {
                for (t, xb) in izip!(tmp.iter_mut().skip(HALF_SIZE), bb[y].iter().copied()) {
                    *t |= xb;
                }
            }
            (false, true) => unreachable!(),
            _ => {
                for (t, yb) in izip!(tmp.iter_mut(), bb[y].iter().copied()) {
                    *t |= yb;
                }
            }
        }
        bb[x] = tmp;
    }
    checked[x] = true;
}
