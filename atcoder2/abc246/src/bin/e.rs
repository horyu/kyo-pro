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
        ss: [Chars; n],
    };
    let ng = ss
        .into_iter()
        .map(|s| s.into_iter().map(|c| c == '#').collect_vec())
        .collect_vec();
    // d[dir][x][y] = dir方向を向いて(x, y)にいるときの移動回数
    let mut ddd = vec![vec![vec![1usize << 60; n]; n]; 4];
    let mut qq = VecDeque::new();
    for dir in 0..4 {
        ddd[dir][ax][ay] = 1;
        qq.push_back((dir, ax, ay));
    }
    while let Some((dir, x, y)) = qq.pop_front() {
        // eprint!("[{dir} {x} {y}] ");
        if (x, y) == (bx, by) {
            println!("{}", ddd[dir][x][y]);
            return;
        }
        for d in 0..4 {
            let nx = x as isize + [1, 1, -1, -1][d];
            let ny = y as isize + [1, -1, 1, -1][d];
            if !(0..(n as isize)).contains(&nx)
                || !(0..(n as isize)).contains(&ny)
                || ng[nx as usize][ny as usize]
            {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            let new_d = ddd[dir][x][y] + usize::from(dir != d);
            if new_d < ddd[d][nx][ny] {
                ddd[d][nx][ny] = new_d;
                if d == dir {
                    qq.push_front((d, nx, ny));
                } else {
                    qq.push_back((d, nx, ny));
                }
            }
        }
    }
    println!("-1");
}
