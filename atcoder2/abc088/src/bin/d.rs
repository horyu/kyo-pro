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
        sss: [Chars; h],
    };
    let sss = sss
        .into_iter()
        .map(|ss| ss.into_iter().map(|s| s == '.').collect_vec())
        .collect_vec();
    let mut shortest = std::usize::MAX;
    let mut qq = VecDeque::new();
    let mut pushed = HashSet::new();
    qq.push_back((0, 0, 0));
    pushed.insert((0, 0));
    while let Some((qi, qj, qd)) = qq.pop_front() {
        let mut xxyy = vec![];
        if 0 < qi {
            xxyy.push((qi - 1, qj));
        }
        if qi < h - 1 {
            xxyy.push((qi + 1, qj));
        }
        if 0 < qj {
            xxyy.push((qi, qj - 1));
        }
        if qj < w - 1 {
            xxyy.push((qi, qj + 1));
        }
        for (x, y) in xxyy {
            if (h - 1, w - 1) == (x, y) {
                shortest = qd + 1;
                qq.clear();
                break;
            }
            if sss[x][y] && pushed.insert((x, y)) {
                qq.push_back((x, y, qd + 1));
            }
        }
    }
    if shortest == std::usize::MAX {
        println!("-1");
        return;
    }
    let whites = sss.into_iter().flatten().filter(|&s| s).count();
    let rs = whites - shortest - 1;
    println!("{rs}");
}
