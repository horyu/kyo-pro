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
        (sx, sy): (Usize1, Usize1),
        (gx, gy): (Usize1, Usize1),
    };
    let mut qq = VecDeque::new();
    qq.push_back((sx, sy, 0));
    let next = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut vv = vec![];
        if 0 < i {
            vv.push((i - 1, j));
            if 1 < i && ss[i - 1][j] == '#' && ss[i - 2][j] == '#' {
                vv.push((i - 2, j));
            }
        }
        if i < h - 1 {
            vv.push((i + 1, j));
            if i + 1 < h - 1 && ss[i + 1][j] == '#' && ss[i + 2][j] == '#' {
                vv.push((i + 2, j));
            }
        }
        if 0 < j {
            vv.push((i, j - 1));
            if 1 < j && ss[i][j - 1] == '#' && ss[i][j - 2] == '#' {
                vv.push((i, j - 2));
            }
        }
        if j < w - 1 {
            vv.push((i, j + 1));
            if j + 1 < w - 1 && ss[i][j + 1] == '#' && ss[i][j + 2] == '#' {
                vv.push((i, j + 2));
            }
        }
        vv
    };
    let mut dd = vec![vec![1usize << 60; w]; h];
    while let Some((qx, qy, qd)) = qq.pop_front() {
        if dd[qx][qy] <= qd {
            continue;
        }
        dd[qx][qy] = qd;
        if (qx, qy) == (gx, gy) {
            println!("{qd}");
            return;
        }
        for (x, y) in next(qx, qy) {
            if ss[x][y] == '#' {
                qq.push_back((x, y, qd + 1));
            } else {
                qq.push_front((x, y, qd));
            }
        }
    }
    // println!("{rs}");
}
