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
        k: usize,
        aa: [usize; n],
    };
    if k == 1 {
        println!("{n}");
        return;
    }
    let mut rs = 0usize;
    let mut cnt = 0;
    for r in 0..(n - 1) {
        if aa[r] < aa[r + 1] {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if k - 1 <= cnt {
            rs += 1;
        }
    }
    println!("{rs}");
}
