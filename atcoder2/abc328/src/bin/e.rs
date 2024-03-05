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
        k: usize,
        uuvvww: [(Usize1, Usize1, usize); m],
    };
    let mut rs = usize::MAX;
    'outer: for jj in (0..m).combinations(n - 1) {
        let mut uf = UnionFind::new(n);
        let mut sum = 0;
        for j in jj {
            let (u, v, w) = uuvvww[j];
            if !uf.union(u, v) {
                continue 'outer;
            }
            sum += w;
        }
        rs = rs.min(sum % k);
    }
    println!("{rs}");
}
