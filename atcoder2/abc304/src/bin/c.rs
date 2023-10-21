#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use num_traits::Pow;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        d: usize,
        xxyy: [(isize, isize); n],
    };
    let mut uf = UnionFind::new(n);
    for ((i, (xi, yi)), (j, (xj, yj))) in xxyy.iter().copied().enumerate().tuple_combinations() {
        if xi.abs_diff(xj).pow(2u32) + yi.abs_diff(yj).pow(2u32) <= d * d {
            uf.union(i, j);
        }
    }
    println!("Yes");
    for i in 1..n {
        let rs = ["No", "Yes"][(uf.equiv(0, i)) as usize];
        println!("{rs}");
    }
}
