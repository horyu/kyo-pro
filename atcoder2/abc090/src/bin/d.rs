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
        k: usize,
    };
    if k == 0 {
        println!("{}", n * n);
        return;
    }
    let mut rs = 0usize;
    for div in (k + 1)..=n {
        // k:3 b: 6
        //     a: 0 1 2 3 4 5 6 7 8 9 ..= n
        // amari: 0 1 2 3 4 5 0 1 2 3
        //              ^ ^ ^       ^
        rs += (n / div) * (div - k);
        rs += (n / div * div + k..=n).count();
    }
    println!("{rs}");
}
