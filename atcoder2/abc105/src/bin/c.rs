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
        mut n: isize,
    };
    // -9
    // +1 => -10
    // -2 => -8
    let mut rs = 0;
    for i in 0.. {
        if n == 0 {
            break;
        }
        let div = (-2isize).pow(i);
        if (n / div).is_odd() {
            rs |= 1 << i;
            n -= div;
        }
    }
    println!("{rs:b}");
}
