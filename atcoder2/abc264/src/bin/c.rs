#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h1: usize,
        w1: usize,
        aaa: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        bbb: [[usize; w2]; h2],
    };
    for yy in (0..h1).combinations(h2) {
        for xx in (0..w1).combinations(w2) {
            if yy
                .iter()
                .enumerate()
                .all(|(i, &y)| xx.iter().enumerate().all(|(j, &x)| aaa[y][x] == bbb[i][j]))
            {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
