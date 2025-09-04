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
    let udlr = |i: usize, j: usize| {
        [
            (0 < i).then(|| (i - 1, j)),
            (i < h - 1).then(|| (i + 1, j)),
            (0 < j).then(|| (i, j - 1)),
            (j < w - 1).then(|| (i, j + 1)),
        ]
        .into_iter()
        .flatten()
    };
    let mut rs = 1;
    let mut vvv = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if ss[i][j] != '.' || vvv[i][j] != 0 {
                continue;
            }
            let k = i * w + j + 1;
            // BFS
            let mut cnt = 0;
            let mut qq = VecDeque::new();
            qq.push_back((i, j));
            vvv[i][j] = k;
            while let Some((qi, qj)) = qq.pop_front() {
                cnt += 1;
                if !udlr(qi, qj).all(|(i, j)| ss[i][j] == '.') {
                    continue;
                }
                for (i, j) in udlr(qi, qj) {
                    if vvv[i][j] == k {
                        continue;
                    }
                    vvv[i][j] = k;
                    qq.push_back((i, j));
                }
            }
            // eprintln!("{i} {j} {k} {cnt}");
            // for vv in &vvv {
            //     eprintln!("{vv:?}");
            // }
            rs = rs.max(cnt);
        }
    }
    println!("{rs}");
}
