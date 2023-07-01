#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [Chars; h],
    };
    if ss[0][0] != 's' {
        println!("No");
        return;
    }

    let udlr = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut vv = vec![];
        if 0 < i {
            vv.push((i - 1, j));
        }
        if i < h - 1 {
            vv.push((i + 1, j));
        }
        if 0 < j {
            vv.push((i, j - 1));
        }
        if j < w - 1 {
            vv.push((i, j + 1));
        }
        vv
    };

    let mut qq = VecDeque::new();
    let mut pushed = vec![vec![false; w]; h];
    pushed[0][0] = true;
    qq.push_back((0, 0, 's'));
    while let Some((qi, qj, qc)) = qq.pop_front() {
        for (i, j) in udlr(qi, qj) {
            let c = ss[i][j];
            if !pushed[i][j]
                && matches!(
                    (qc, c),
                    ('s', 'n') | ('n', 'u') | ('u', 'k') | ('k', 'e') | ('e', 's')
                )
            {
                pushed[i][j] = true;
                qq.push_back((i, j, c));
            }
        }
    }
    let rs = ["No", "Yes"][pushed[h - 1][w - 1] as usize];
    println!("{rs}");
}
