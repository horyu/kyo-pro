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
        ax: Usize1,
        ay: Usize1,
        bx: Usize1,
        by: Usize1,
        s: [Chars; n]
    };
    // 01-bfs
    let ng = s
        .into_iter()
        .map(|cc| cc.into_iter().map(|c| c == '#').collect_vec())
        .collect_vec();

    // 右上　右下　左下　左上
    let next = |x: usize, y: usize, dir: usize| -> Option<(usize, usize)> {
        match dir {
            0 => izip!((0..x).rev(), (y + 1)..n).next(),
            1 => izip!((x + 1)..n, (y + 1)..n).next(),
            2 => izip!((x + 1)..n, (0..y).rev()).next(),
            _ => izip!((0..x).rev(), (0..y).rev()).next(),
        }
        .filter(|&(xx, yy)| !ng[xx][yy])
    };

    let mut qq = VecDeque::new();
    let mut dd = vec![vec![[1 << 32; 4]; n]; n];
    let mut checked = vec![vec![[false; 4]; n]; n];
    for dir in 0..4 {
        if let Some((x, y)) = next(ax, ay, dir) {
            dd[x][y][dir] = 1;
            qq.push_back((x, y, dir));
        }
    }

    while let Some((qx, qy, qdir)) = qq.pop_front() {
        if (qx, qy) == (bx, by) {
            println!("{}", dd[qx][qy][qdir]);
            return;
        }
        if checked[qx][qy][qdir] {
            continue;
        }
        checked[qx][qy][qdir] = true;

        for dir in 0..4 {
            let Some((x, y)) = next(qx, qy, dir) else { continue };
            let new_d = dd[qx][qy][qdir] + (qdir != dir) as usize;

            if new_d < dd[x][y][dir] {
                dd[x][y][dir] = new_d;
                if qdir == dir {
                    qq.push_front((x, y, dir));
                } else {
                    qq.push_back((x, y, dir));
                }
            }
        }
    }
    println!("-1");
}
