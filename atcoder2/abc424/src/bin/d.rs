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
    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            mut ss: [Chars; h],
        };
        let mut rs = 0;
        let tes = |(x, y): (usize, usize)| -> bool {
            (0..2).all(|i| (0..2).all(|j| ss[x + i][y + j] == '#'))
        };
        for i in 0..(h - 1) {
            for j in 0..(w - 1) {
                let arr = [(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)];
                if arr.iter().all(|&(x, y)| ss[x][y] == '#') {
                    let x = i + usize::from(i < h - 1);
                    let y = j + 1;
                    ss[x][y] = '.';
                    rs += 1;
                }
            }
        }
        println!("{rs}");
    }
}
