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
    };
    let mut rs = 0;
    for i in 0..m {
        let mut uf = UnionFind::new(n);
        for (j, (a, b)) in aabb.iter().copied().enumerate() {
            if i == j {
                continue;
            }
            uf.union(a, b);
        }
        let (a, b) = aabb[i];
        if uf.union(a, b) {
            rs += 1;
        }
    }
    println!("{rs}");
}
