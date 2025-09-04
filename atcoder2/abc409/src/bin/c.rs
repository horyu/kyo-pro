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
        l: usize,
        dd: [usize; n - 1],
    };
    if l % 3 != 0 {
        println!("0");
        return;
    }
    let mut cc = vec![0usize; l];
    let mut pos = 0;
    cc[pos] += 1;
    for d in dd {
        pos += d;
        pos %= l;
        cc[pos] += 1;
    }
    // eprintln!("{cc:?}");
    let mut rs = 0;
    let ll = l / 3;
    for i in 0..ll {
        rs += cc[i] * cc[i + ll] * cc[i + 2 * ll];
    }
    println!("{rs}");
}
