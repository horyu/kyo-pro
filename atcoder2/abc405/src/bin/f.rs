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
        m: usize,
        aabb: [(Usize1, Usize1); m],
        q: usize,
        ccdd: [(Usize1, Usize1); q],
    };
    let n2 = n * 2;
    let mut ft = ac_library::FenwickTree::new(n2 * 2, 0isize);
    for (a, b) in aabb.iter().copied() {
        if b - a < n {
            ft.add(a, 1);
            ft.add(b, -1);
        } else {
            ft.add(b, 1);
            ft.add(a + n2, -1);
        }
    }
    for (c, d) in ccdd.iter().copied() {
        let rs = if d - c < n {
            ft.sum(c..d).abs()
        } else {
            ft.sum(d..(c + n2)).abs()
        };
        println!("{rs}");
    }
}
