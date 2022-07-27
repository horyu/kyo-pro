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
        h: usize,
        w: usize,
        mut ccc: [[usize; 10]; 10],
        aaa: [[isize; w]; h]
    };
    for (k, i, j) in iproduct!(0..10, 0..10, 0..10) {
        ccc[i][j] = ccc[i][j].min(ccc[i][k] + ccc[k][j]);
    }
    let mut rs = 0usize;
    for aa in aaa {
        for a in aa {
            if a != -1 {
                rs += ccc[a as usize][1];
            }
        }
    }
    println!("{rs}");
}
