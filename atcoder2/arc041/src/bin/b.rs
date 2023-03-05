#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        bbb: [Bytes; n],
    };
    let mut bbb = bbb
        .into_iter()
        .map(|bb| bb.into_iter().map(|b| b - b'0').collect_vec())
        .collect_vec();
    let mut rs = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            let b = bbb[i][j];
            if 0 < b {
                rs[i + 1][j] = b;
                bbb[i][j] -= b;
                bbb[i + 2][j] -= b;
                bbb[i + 1][j - 1] -= b;
                bbb[i + 1][j + 1] -= b;
            }
        }
    }
    for rs in rs {
        let rs = rs.iter().join("");
        println!("{rs}");
    }
}
