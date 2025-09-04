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
        n: usize,
        aaa: [Chars; n],
    };
    let mut bbb = aaa.clone();
    for d in 0..(n / 2) {
        // 外側から距離iの部分を -45*(d + 1)度回転させる
        // 上辺・下辺
        for i in [d, n - 1 - d] {
            // eprintln!("{d} {i} {:?}", d..=(n - 1 - d));
            for j in d..=(n - 1 - d) {
                let (mut x, mut y) = (i, j);
                for _ in 0..=(d % 4) {
                    (x, y) = (y, n - 1 - x);
                }
                bbb[x][y] = aaa[i][j];
            }
        }
        // 左辺・右辺
        for j in [d, n - 1 - d] {
            // eprintln!("{d} {:?} {j}", d..=(n - 1 - d));
            for i in d..=(n - 1 - d) {
                let (mut x, mut y) = (i, j);
                for _ in 0..=(d % 4) {
                    (x, y) = (y, n - 1 - x);
                }
                bbb[x][y] = aaa[i][j];
            }
        }
    }
    for bb in bbb {
        let rs = bb.iter().join("");
        println!("{rs}");
    }
}
