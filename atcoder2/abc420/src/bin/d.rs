#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use num_traits::ops::checked;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        h: usize,
        w: usize,
        aaa: [Chars; h],
    };
    let mut mats = [aaa.clone(), aaa];
    let mut si = 0;
    let mut sj = 0;
    for i in 0..h {
        for j in 0..w {
            match mats[0][i][j] {
                'S' => {
                    si = i;
                    sj = j;
                    mats[0][i][j] = '.';
                    mats[1][i][j] = '.';
                }
                'o' => {
                    mats[0][i][j] = '.';
                    mats[1][i][j] = '#';
                }
                'x' => {
                    mats[0][i][j] = '#';
                    mats[1][i][j] = '.';
                }
                _ => (),
            }
        }
    }
    let mut qq = VecDeque::new();
    let mut checked = vec![vec![vec![false; w]; h]; 2];
    checked[0][si][sj] = true;
    qq.push_back((si, sj, 0usize, 0));
    while let Some((qi, qj, qk, qd)) = qq.pop_front() {
        for (di, dj) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let ni = qi.wrapping_add(di);
            let nj = qj.wrapping_add(dj);
            if h <= ni || w <= nj || checked[qk][ni][nj] {
                continue;
            }
            checked[qk][ni][nj] = true;
            let nd = qd + 1;
            match mats[qk][ni][nj] {
                'G' => {
                    println!("{nd}");
                    return;
                }
                '.' => {
                    qq.push_back((ni, nj, qk, nd));
                }
                '?' => {
                    qq.push_back((ni, nj, qk ^ 1, nd));
                }
                _ => (),
            }
        }
    }
    println!("-1");
}
