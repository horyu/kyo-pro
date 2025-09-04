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
        q: usize,
        xx: [Usize1; q],
    };
    let mut a2i = (0..n).collect_vec();
    let mut i2a = (0..n).collect_vec();
    for x in xx {
        let i = a2i[x];
        let j = if i == n - 1 { i - 1 } else { i + 1 };
        let y = i2a[j];
        a2i.swap(x, y);
        i2a.swap(i, j);
    }
    let rs = i2a.into_iter().map(|a| a + 1).join(" ");
    println!("{rs}");
}
