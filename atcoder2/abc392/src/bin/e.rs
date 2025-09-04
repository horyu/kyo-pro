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
        aabb: [(Usize1, Usize1); m],
    };
    let mut dsu = ac_library::Dsu::new(n);
    let mut dups = vec![];
    for (i, (a, b)) in aabb.iter().copied().enumerate() {
        if dsu.same(a, b) {
            dups.push(i);
        } else {
            dsu.merge(a, b);
        }
    }
    let mut hs = HashSet::new();
    for i in 0..n {
        hs.insert(dsu.leader(i));
    }
    let mut rrss = vec![];
    for i in dups {
        if hs.len() == 1 {
            break;
        }
        let (a, b) = aabb[i];
        let la = dsu.leader(a);
        // b側を hs の適当なノードに繋ぎ変える
        let lx = hs.iter().copied().find(|&x| x != la).unwrap();
        rrss.push((i + 1, a + 1, lx + 1));
        let l = dsu.merge(la, lx);
        hs.remove(&la);
        hs.remove(&lx);
        hs.insert(l);
    }
    println!("{}", rrss.len());
    for (i, p, q) in rrss {
        println!("{i} {p} {q}");
    }
}
