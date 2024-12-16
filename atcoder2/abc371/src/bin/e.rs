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
        n: usize,
        aa: [usize; n],
    };
    let mut mm = multimap::MultiMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        mm.insert(a, i + 1);
    }
    // 区間 1 => 1, 2 => 3, 3 => 6
    let f = |x: usize| -> usize { x * (x + 1) / 2 };
    let mut rs = 0;

    for (a, ii) in mm {
        rs += f(n);

        for (l, r) in chain!([0], ii, [n + 1]).tuple_windows() {
            rs -= f(r - l - 1);
        }
    }
    println!("{rs}");
}
