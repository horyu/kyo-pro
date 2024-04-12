#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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

    let mut dsu = ac_library::Dsu::new(h * w);
    let mut reds = vec![];
    for (i, ss) in sss.iter().enumerate() {
        for (j, &s) in ss.iter().enumerate() {
            if s == '.' {
                reds.push((i, j));
            } else {
                // 上下左右の # であるマスを union
                for (x, y) in udlr(i, j) {
                    if sss[x][y] == '#' {
                        dsu.merge(i * w + j, x * w + y);
                    }
                }
            }
        }
    }

    let ttt = sss
        .into_iter()
        .enumerate()
        .map(|(i, ss)| {
            ss.into_iter()
                .enumerate()
                .map(|(j, s)| (s == '#').then(|| 1 + dsu.leader(i * w + j)).unwrap_or(0))
                .collect_vec()
        })
        .collect_vec();
    // for tt in &ttt {
    //     eprintln!("{tt:?}");
    // }

    let green_cnt = ttt.iter().flatten().filter(|&&t| t != 0).unique().count();
    let bunbo = reds.len();
    let mut bunsi = ModInt998244353::default();
    for (i, j) in reds {
        let mawari = udlr(i, j)
            .map(|(x, y)| ttt[x][y])
            .filter(|&t| t != 0)
            .unique()
            .count();
        if mawari == 0 {
            bunsi += green_cnt + 1;
        } else {
            bunsi += green_cnt + 1 - mawari;
        }
        // eprintln!("{i} {j} {bunsi}");
    }

    let rs = bunsi / bunbo;
    println!("{rs}");
}
