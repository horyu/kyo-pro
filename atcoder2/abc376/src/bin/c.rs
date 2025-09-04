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
        mut aa: [isize; n],
        mut bb: [isize; n - 1],
    };
    aa.sort_unstable();
    bb.sort_unstable();
    let mut rs = None;
    while let (Some(&a), Some(&b)) = (aa.last(), bb.last()) {
        if a <= b {
            aa.pop();
            bb.pop();
        } else {
            if rs.is_none() {
                rs = Some(a);
                aa.pop();
            } else {
                println!("-1");
                return;
            }
        }
    }
    let rs = rs.or(aa.pop()).unwrap();
    println!("{rs}");
}
