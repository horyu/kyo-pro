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
        xxyy: [(isize, isize); n],
    };
    for i in 0..n {
        let mut len = 0;
        let mut rs = !0;
        let (xi, yi) = xxyy[i];
        for j in 0..n {
            if i == j {
                continue;
            }
            let (xj, yj) = xxyy[j];
            let tmp = (xi - xj).pow(2) + (yi - yj).pow(2);
            if len < tmp {
                len = tmp;
                rs = j + 1;
            }
        }
        println!("{rs}");
    }
}
