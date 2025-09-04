#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        pp: [usize; n],
    };
    let mut rrss = vec![0; n];
    let mut r = 1;
    while r <= n {
        let ii = rrss
            .iter()
            .copied()
            .positions(|x| x == 0)
            .max_set_by_key(|&i| pp[i]);
        let k = ii.len();
        for i in ii {
            rrss[i] = r;
        }
        r += k;
    }
    let rs = rrss.iter().join("\n");
    println!("{rs}");
}
