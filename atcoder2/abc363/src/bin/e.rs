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
        y: usize,
        aaa: [[usize; w]; h],
    };
    let mut pushed = vec![vec![false; w]; h];
    let mut qqq = vec![VecDeque::default(); 1e5 as usize + 1];
    // aaaの外枠をmmに追加
    for i in [0, h - 1] {
        for j in 0..w {
            if !pushed[i][j] {
                pushed[i][j] = true;
                qqq[aaa[i][j]].push_back((i, j));
            }
        }
    }
    for j in [0, w - 1] {
        for i in 0..h {
            if !pushed[i][j] {
                pushed[i][j] = true;
                qqq[aaa[i][j]].push_back((i, j));
            }
        }
    }

    let hh = h as isize;
    let ww = w as isize;
    let mut cnt = h * w;
    for y in 1..=y {
        let mut qq = std::mem::take(&mut qqq[y]);
        while let Some((i, j)) = qq.pop_front() {
            cnt -= 1;
            for &(di, dj) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni < 0 || hh <= ni || nj < 0 || ww <= nj {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if pushed[ni][nj] {
                    continue;
                }
                pushed[ni][nj] = true;
                let a = aaa[ni][nj];
                if a <= y {
                    qq.push_back((ni, nj));
                } else {
                    qqq[a].push_back((ni, nj));
                }
            }
        }
        println!("{cnt}");
    }
}
