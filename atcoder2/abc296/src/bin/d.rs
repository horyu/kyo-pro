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
        m: usize,
    };
    let mut rs = !0;
    for a in 1..=(m.sqrt() + 1) {
        let b = m.div_ceil(a);
        if a <= n && b <= n && m <= a * b {
            rs = rs.min(a * b);
        }
    }
    if rs == !0 {
        println!("-1");
    } else {
        println!("{rs}");
    }
}
