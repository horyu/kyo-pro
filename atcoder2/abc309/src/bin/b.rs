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
    let mut rs = aaa.clone();
    for (i, aa) in aaa.iter().enumerate() {
        for (j, a) in aa.iter().copied().enumerate() {
            if i == 0 && j < n - 1 {
                rs[i][j + 1] = a;
            } else if j == n - 1 && i < n - 1 {
                rs[i + 1][j] = a;
            } else if i == n - 1 && 0 < j {
                rs[i][j - 1] = a;
            } else if j == 0 && 0 < i {
                rs[i - 1][j] = a;
            }
        }
    }
    for rs in rs {
        println!("{}", rs.iter().join(""));
    }
}
