#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
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
        t: usize,
    };
    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            ss: [Chars; h],
        };
        // マスの辺を以下のように番号付けする
        // ┏0┓
        // 1 2
        // ┗3┛
        // A: 　  0-3, 1-2
        // B: ＼  0-2, 1-3
        // C: ／  0-1, 2-3
        // 現在繋がっている辺は距離0、繋がっていない辺は距離1として [0][0][1] から [h-1][w-1][2] までの最短距離を求める
        let s_to_ee = |c: char| -> ([usize; 2], [usize; 2]) {
            match c {
                'A' => ([0, 3], [1, 2]),
                'B' => ([0, 2], [1, 3]),
                _ => ([0, 1], [2, 3]),
            }
        };
        let to_pos_vec = |i: usize, j: usize, e: usize| -> Vec<(usize, usize, usize)> {
            let mut vv = vec![(i, j, e)];
            match e {
                0 if 0 < i => vv.push((i - 1, j, 3)),
                1 if 0 < j => vv.push((i, j - 1, 2)),
                2 if j + 1 < w => vv.push((i, j + 1, 1)),
                3 if i + 1 < h => vv.push((i + 1, j, 0)),
                _ => {}
            };
            vv
        };
        let mut qq = VecDeque::new();
        let mut dist = vec![vec![vec![!0usize; 4]; w]; h];
        qq.push_back(((0, 0, 1), 0)); // (i, j, edge, dist)
        dist[0][0][1] = 0;
        while let Some(((i, j, e), d)) = qq.pop_front() {
            if dist[i][j][e] < d {
                continue;
            }
            if (i, j, e) == (h - 1, w - 1, 2) {
                println!("{d}");
                break;
            }
            let (xx, yy) = s_to_ee(ss[i][j]);
            let (ff, bb) = if xx.contains(&e) { (xx, yy) } else { (yy, xx) };
            for fe in ff {
                for (ni, nj, ne) in to_pos_vec(i, j, fe) {
                    if dist[ni][nj][ne] <= d {
                        continue;
                    }
                    dist[ni][nj][ne] = d;
                    qq.push_front(((ni, nj, ne), d));
                }
            }
            for be in bb {
                for (ni, nj, ne) in to_pos_vec(i, j, be) {
                    if dist[ni][nj][ne] <= d + 1 {
                        continue;
                    }
                    dist[ni][nj][ne] = d + 1;
                    qq.push_back(((ni, nj, ne), d + 1));
                }
            }
        }
    }
}
