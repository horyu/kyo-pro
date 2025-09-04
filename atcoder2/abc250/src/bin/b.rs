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
        a: usize,
        b: usize,
    };
    let mut rs = vec![vec!['.'; b * n]; a * n];
    for i in 0..(a * n) {
        for j in 0..(b * n) {
            let ii = i / a;
            let jj = j / b;
            if (ii + jj).is_odd() {
                rs[i][j] = '#';
            }
        }
    }

    for rs in rs {
        println!("{}", rs.iter().join(""));
    }
}
