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
    };
    let mut c = vec![vec!['#'; n]; n];
    for d in 1..(n.div_ceil(2)) {
        if d % 2 == 0 {
            continue;
        }
        for i in [d, n - 1 - d] {
            for j in d..=(n - 1 - d) {
                c[i][j] = '.';
                c[j][i] = '.';
            }
        }
    }
    let rs = c
        .into_iter()
        .map(|x| x.into_iter().collect::<String>())
        .join("\n");
    println!("{rs}");
}
