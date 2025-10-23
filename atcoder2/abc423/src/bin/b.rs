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

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        ll: [usize; n],
    };
    let mut vv = vec![1; n + 1];
    vv[0] = 0;
    vv[n] = 0;
    for (i, l) in ll.iter().copied().enumerate() {
        if l == 1 {
            break;
        }
        vv[i + 1] = 0;
    }
    for (i, l) in ll.iter().copied().enumerate().rev() {
        if l == 1 {
            break;
        }
        vv[i] = 0;
    }
    // eprintln!("{:?}", &vv);
    let rs = vv.iter().sum::<usize>();
    println!("{rs}");
}
