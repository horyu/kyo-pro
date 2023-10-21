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
        n: usize,
        m: usize,
    };
    let mut dd = vec![];
    for _ in 0..m {
        input! {
            c: usize,
            aa: [Usize1; c],
        };
        let d = aa.iter().fold(0u16, |acc, a| acc | (1 << a));
        dd.push(d);
    }
    let mut rs = 0;
    for size in 1..=m {
        for dd in dd.iter().copied().combinations(size) {
            let d = dd.iter().fold(0, |acc, d| acc | d);
            if d == (1 << n) - 1 {
                rs += 1;
            }
        }
    }
    println!("{rs}");
}
