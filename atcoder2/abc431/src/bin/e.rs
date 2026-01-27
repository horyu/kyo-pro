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
        // B: ／  0-1, 2-3
        // C: ＼  0-2, 1-3
        // 現在繋がっている辺は距離0、繋がっていない辺は距離1として [0][0][1] から [h-1][w-1][2] までの最短距離を求める
        let s_to_ee = |c: char| -> ([usize; 2], [usize; 2]) {
            match c {
                'A' => ([0, 3], [1, 2]),
                'B' => ([0, 1], [2, 3]),
                _ => ([0, 2], [1, 3]),
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
        // 0-1 BFS
        let mut pushed = vec![vec![vec![false; 4]; w]; h];
        let mut qq = VecDeque::new();
        qq.push_back(((0, 0, 1), 0)); // (i, j, edge, dist)
        pushed[0][0][1] = true;
        while let Some(((i, j, e), d)) = qq.pop_front() {
            if (i, j, e) == (h - 1, w - 1, 2) {
                println!("{d}");
                break;
            }
            let (xx, yy) = s_to_ee(ss[i][j]);
            let (ff, bb) = if xx.contains(&e) { (xx, yy) } else { (yy, xx) };
            for f in ff {
                if pushed[i][j][f] {
                    continue;
                }
                pushed[i][j][f] = true;
                // TODO
            }
            for b in bb {
                if pushed[i][j][b] {
                    continue;
                }
                pushed[i][j][b] = true;
                // TODO
            }
        }
    }
    // println!("{rs}");
}
