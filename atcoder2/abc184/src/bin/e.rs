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
        aaa: [Chars; h],
    };
    let (mut si, mut sj) = (0, 0);
    let (mut gi, mut gj) = (0, 0);
    let mut hm = HashMap::new();
    for (i, aa) in aaa.iter().enumerate() {
        for (j, a) in aa.iter().copied().enumerate() {
            match a {
                'S' => {
                    (si, sj) = (i, j);
                }
                'G' => {
                    (gi, gj) = (i, j);
                }
                'a'..='z' => {
                    hm.entry(a).or_insert_with(Vec::new).push((i, j));
                }
                _ => (),
            }
        }
    }
    let udlr = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut vv = vec![];
        if 0 < i {
            vv.push((i - 1, j));
        }
        if i < h - 1 {
            vv.push((i + 1, j));
        }
        if 0 < j {
            vv.push((i, j - 1));
        }
        if j < w - 1 {
            vv.push((i, j + 1));
        }
        vv.retain(|&(vi, vj)| aaa[vi][vj] != '#');
        vv
    };
    let mut qq = VecDeque::new();
    let mut pushed = vec![vec![false; w]; h];
    qq.push_back((si, sj, 0));
    pushed[si][sj] = true;
    while let Some((qi, qj, qd)) = qq.pop_front() {
        if (qi, qj) == (gi, gj) {
            println!("{qd}");
            return;
        }
        let qa = aaa[qi][qj];
        for (i, j) in chain!(udlr(qi, qj), hm.remove(&qa).unwrap_or_default()) {
            if pushed[i][j] {
                continue;
            }
            pushed[i][j] = true;
            qq.push_back((i, j, qd + 1));
        }
    }
    println!("-1");
}
