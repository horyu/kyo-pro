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
        ss: [Chars; h],
    };
    let mut rs = 0;
    let mut pushed = HashSet::new();
    let dd = iproduct!(-1..=1, -1..=1)
        .filter(|&(di, dj)| di != 0 || dj != 0)
        .collect_vec();
    for i in 0..h {
        for j in 0..w {
            if pushed.insert((i, j)) && ss[i][j] == '#' {
                rs += 1;
                let mut qq = VecDeque::new();
                qq.push_back((i, j));
                while let Some((qi, qj)) = qq.pop_front() {
                    // 回り８マスを探索
                    for (di, dj) in dd.iter().copied() {
                        let (ni, nj) = (qi as isize + di, qj as isize + dj);
                        if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                            continue;
                        }
                        let (ni, nj) = (ni as usize, nj as usize);
                        if pushed.insert((ni, nj)) && ss[ni][nj] == '#' {
                            qq.push_back((ni, nj));
                        }
                    }
                }
            }
        }
    }
    println!("{rs}");
}
