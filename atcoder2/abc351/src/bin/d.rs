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
        h: usize,
        w: usize,
        ss: [Chars; h],
    };
    let udlr = |i: usize, j: usize| {
        [
            (0 < i).then(|| (i - 1, j)),
            (i < h - 1).then(|| (i + 1, j)),
            (0 < j).then(|| (i, j - 1)),
            (j < w - 1).then(|| (i, j + 1)),
        ]
        .into_iter()
        .flatten()
    };
    let mut dsu = ac_library::Dsu::new(h * w);
    let mut ppqq = vec![];
    for i in 0..h {
        for j in 0..w {
            if ss[i][j] != '.' {
                continue;
            }
            // #に隣接していたら後から接続
            if !udlr(i, j).all(|(x, y)| ss[x][y] == '.') {
                ppqq.push((i, j));
                continue;
            }
            // 右マスも#に隣接していなければ接続
            if j < w - 1 && udlr(i, j + 1).all(|(x, y)| ss[x][y] == '.') {
                dsu.merge(i * w + j, i * w + j + 1);
            }
            // 下マスも#に隣接していなければ接続
            if i < h - 1 && udlr(i + 1, j).all(|(x, y)| ss[x][y] == '.') {
                dsu.merge(i * w + j, (i + 1) * w + j);
            }
        }
    }
    let mut counter = counter::Counter::<_>::new();
    // let mut rs = 1usize;
    // let mut ccc = vec![vec![0; w]; h];
    for aa in dsu.groups() {
        let b = dsu.leader(aa[0]);
        // eprintln!("{b} {aa:?}");
        let (bi, bj) = (b / w, b % w);
        if ss[bi][bj] == '#' {
            continue;
        }
        counter[&b] += aa.len();
    }
    for (p, q) in ppqq {
        // 隣接している.マスのleader
        let mut targets = vec![];
        for (x, y) in udlr(p, q) {
            let l = dsu.leader(x * w + y);
            if counter.contains_key(&l) {
                targets.push(l);
            }
        }
        targets.sort_unstable();
        targets.dedup();
        for t in targets {
            let (ti, tj) = (t / w, t % w);
            if !udlr(ti, tj).all(|(x, y)| ss[x][y] == '.') {
                continue;
            }
            counter[&t] += 1;
        }
    }
    let rs = counter.values().max().unwrap_or(&1);
    println!("{rs}");
}
