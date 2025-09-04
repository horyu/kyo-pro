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
        aa: [usize; n],
    };
    // ll[i] = 0..=i までに作成できる競技単調増加列の最大長
    let mut ll = vec![1; n];
    // rr[i] = i..n までに作成できる競技単調増加列の最大長
    let mut rr = vec![1; n];
    for i in 1..n {
        if ll[i - 1] < aa[i] {
            ll[i] = ll[i - 1] + 1;
        } else {
            ll[i] = aa[i];
        }
    }
    for i in (0..(n - 1)).rev() {
        if rr[i + 1] < aa[i] {
            rr[i] = rr[i + 1] + 1;
        } else {
            rr[i] = aa[i];
        }
    }
    let mut rs = 1;
    for (l, r) in izip!(ll, rr) {
        rs = rs.max(l.min(r));
    }
    println!("{rs}");
}
