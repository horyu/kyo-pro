#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        d: usize,
    };
    if (n - 1) / 2 < d {
        println!("No");
        return;
    }
    println!("Yes");
    let mut m = 0;
    let mut rrss = vec![(0, 0); n * d + 1];
    for i in 1..=n {
        for k in 1..=d {
            m += 1;
            rrss[m].0 = i;
            rrss[m].1 = if i + k <= n { i + k } else { i + k - n };
        }
    }
    for (a, b) in rrss.into_iter().skip(1).take(m) {
        println!("{a} {b}");
    }
}
