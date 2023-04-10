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
        h: usize,
        w: usize,
        ccc: [Chars; h],
    };
    let mut ngs = vec![vec![false; w]; h];
    let mut si = 0;
    let mut sj = 0;
    for (i, cc) in ccc.into_iter().enumerate() {
        for (j, c) in cc.into_iter().enumerate() {
            match c {
                'S' => {
                    si = i;
                    sj = j;
                    ngs[i][j] = true;
                }
                '#' => {
                    ngs[i][j] = true;
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
        vv.retain(|&(vi, vj)| !ngs[vi][vj]);
        vv
    };
    for (ti, tj) in udlr(si, sj) {
        let mut qq = VecDeque::new();
        let mut pushed = HashSet::new();
        qq.push_back((ti, tj));
        pushed.insert((ti, tj));
        while let Some((qi, qj)) = qq.pop_front() {
            if (qi, qj) != (ti, tj) && 1 == qi.abs_diff(si) + qj.abs_diff(sj) {
                println!("Yes");
                return;
            }
            for (i, j) in udlr(qi, qj) {
                if pushed.insert((i, j)) {
                    qq.push_back((i, j));
                }
            }
        }
    }
    println!("No");
}
