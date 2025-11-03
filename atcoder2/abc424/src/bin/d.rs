#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        t: usize,
    };
    let arr = |i: usize, j: usize| [(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)];
    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            mut ss: [Chars; h],
        };
        let mut ccc = vec![vec![0; w]; h];
        let mut hs = HashSet::new();
        for i in 0..(h - 1) {
            for j in 0..(w - 1) {
                if arr(i, j).into_iter().all(|(x, y)| ss[x][y] == '#') {
                    hs.insert((i, j));
                    for &(x, y) in &arr(i, j) {
                        ccc[x][y] += 1;
                    }
                }
            }
        }
        // c=1 である端っこのマスから優先的に潰す
        let mut rs = 0;
        while !hs.is_empty() {
            let mut new_hs = hs.clone();
            let mut tmp1122 = None;
            for (i, j) in hs.iter().copied() {
                if arr(i, j).into_iter().any(|(x, y)| ss[x][y] == '.') {
                    for (x, y) in arr(i, j) {
                        new_hs.remove(&(x, y));
                        ccc[x][y] -= 1;
                    }
                    continue;
                }
                let mut c = 0;
                for (x, y) in arr(i, j) {
                    if ccc[x][y] == 1 {
                        c += 1;
                    }
                }
                if c == 4 {
                    // 11
                    // 11
                    // どこでも良い
                    ss[i][j] = '.';
                    for (x, y) in arr(i, j) {
                        ccc[x][y] = 0;
                        new_hs.remove(&(x, y));
                    }
                    rs += 1;
                } else if c == 3 {
                    // 1*
                    // 11
                    // 1ではない部分を潰す
                    for (x, y) in arr(i, j) {
                        if ccc[x][y] != 1 {
                            ss[x][y] = '.';
                        }
                        ccc[x][y] -= 1;
                        new_hs.remove(&(x, y));
                    }
                    rs += 1;
                } else if c == 2 {
                    // 11; 22 だった場合には後回しにする
                    if arr(i, j)
                        .into_iter()
                        .map(|(x, y)| ccc[x][y])
                        .sorted_unstable()
                        .collect_vec()
                        == vec![1, 1, 2, 2]
                    {
                        tmp1122 = Some((i, j));
                    }
                } else if c == 1 {
                    // **
                    // *1
                    // 一番大きい部分を潰す
                    let Some((x, y)) = arr(i, j).into_iter().max_by_key(|&(x, y)| ccc[x][y]) else {
                        unreachable!();
                    };
                    ss[x][y] = '.';
                    for (x, y) in arr(i, j) {
                        ccc[x][y] -= 1;
                        new_hs.remove(&(x, y));
                    }
                    rs += 1;
                }
            }
            if hs.len() == new_hs.len() {
                // 変化がなかった -> 1122 処理
                if let Some((i, j)) = tmp1122 {
                    // 2の部分を潰す
                    let Some((x, y)) = arr(i, j)
                        .into_iter()
                        .find(|&(x, y)| ccc[x][y] == 2)
                    else {
                        unreachable!();
                    };
                    ss[x][y] = '.';
                    for (x, y) in arr(i, j) {
                        ccc[x][y] -= 1;
                        new_hs.remove(&(x, y));
                    }
                    rs += 1;
                } else {
                    unreachable!();
                }
            }
            hs = new_hs;
        }

        println!("{rs}");
    }
}
