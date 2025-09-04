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
        m: usize,
        d: usize,
        mut yy: usize,
        mut mm: usize,
        mut dd: usize,
    };
    dd += 1;
    if d < dd {
        dd = 1;
        mm += 1;
    }
    if m < mm {
        mm = 1;
        yy += 1;
    }
    println!("{yy} {mm} {dd}");
}
