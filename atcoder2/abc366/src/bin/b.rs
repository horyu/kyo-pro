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
        ss: [Chars; n],
    };
    let m = ss.iter().map(|s| s.len()).max().unwrap();
    let mut rs = vec![vec!['*'; n]; m];
    for (i, s) in ss.into_iter().enumerate() {
        for (j, c) in s.into_iter().enumerate() {
            rs[j][n - 1 - i] = c;
        }
    }
    for mut rs in rs {
        while let Some('*') = rs.last() {
            rs.pop();
        }
        let rs = rs.into_iter().collect::<String>();
        println!("{rs}");
    }
}
