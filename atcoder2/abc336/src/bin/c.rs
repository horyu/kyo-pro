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
        mut n: usize,
    };
    if n == 1 {
        println!("0");
        return;
    }
    n -= 1;
    let mut vv = vec![];
    while 0 < n {
        vv.push(n % 5 * 2);
        n /= 5;
    }
    let rs = vv.into_iter().rev().join("");
    println!("{rs}");
}
