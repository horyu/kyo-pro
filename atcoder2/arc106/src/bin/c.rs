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
        n: isize,
        m: isize,
    };
    if m < 0 || m == n || (1 < n && n == m + 1) {
        println!("-1");
        return;
    }
    let n = n as usize;
    let m = m as usize;
    for i in 1..=(n - 1) {
        println!("{} {}", 3 * i, 3 * i + 1);
    }
    println!("1 {}", 3 * (m + 2) - 1);
}
