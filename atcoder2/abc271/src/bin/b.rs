#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut aaa = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            aa: [usize; l]
        };
        aaa.push(aa);
    }
    for _ in 0..q {
        input! {s: Usize1, t: Usize1};
        println!("{}", aaa[s][t]);
    }
}