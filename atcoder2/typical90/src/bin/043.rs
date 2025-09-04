#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        rs: Usize1,
        cs: Usize1,
        rt: Usize1,
        ct: Usize1,
        sss: [Chars; h],
    };
    let mut ddd = vec![vec![vec![usize::MAX; 4]; w]; h];
    let mut qq = VecDeque::new();
    for dir in 0..4 {
        ddd[rs][cs][dir] = 0;
        qq.push_back((rs, cs, dir, 0));
    }
    while let Some((qx, qy, qdir, qcost)) = qq.pop_front() {
        if (qx, qy) == (rt, ct) {
            println!("{qcost}");
            return;
        }
        for (dir, (dx, dy)) in [(1, 0), (0, -1), (-1, 0), (0, 1)]
            .iter()
            .copied()
            .enumerate()
        {
            let (nx, ny) = (qx as isize + dx, qy as isize + dy);
            if !(0..(h as isize)).contains(&nx) || !(0..(w as isize)).contains(&ny) {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if sss[nx][ny] == '#' {
                continue;
            }
            let ncost = qcost + usize::from(qdir != dir);
            if ddd[nx][ny][dir] <= ncost {
                continue;
            }
            ddd[nx][ny][dir] = ncost;
            if qdir == dir {
                qq.push_front((nx, ny, dir, ncost));
            } else {
                qq.push_back((nx, ny, dir, ncost));
            }
        }
    }
}
