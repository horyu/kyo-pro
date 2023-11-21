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
        ss: [Chars; n],
    };
    let mut ss = ss
        .into_iter()
        .map(|s| s.into_iter().map(|c| c == 'o').collect_vec())
        .chain([vec![true; n]])
        .collect_vec();
    let mut rs = 0;
    for i in 0..n {
        if let Some(j) = ss[i].iter().copied().rposition(|tf| !tf) {
            rs += 1;
            for jj in j..n {
                ss[i + 1][jj] = true;
            }
        }
    }
    println!("{rs}");
}
