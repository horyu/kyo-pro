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
        q: usize,
        ccdd: [(Usize1, Usize1); q],
    };
    // https://atcoder.jp/contests/abc405/editorial/13011
    let n2 = n * 2;
    let mut ft = ac_library::FenwickTree::new(n2 * 2, 0isize);

    let mut pp = vec![!0; n2];
    for (i, (a, b)) in aabb.iter().copied().enumerate() {
        pp[a] = i;
        pp[b] = i;
    }
    let mut gg = vec![vec![]; n2];
    for (j, (c, d)) in ccdd.iter().copied().enumerate() {
        gg[d].push(j);
    }
    let mut rrss = vec![0; q];
    for i in 0..n2 {
        if pp[i] != !0 {
            ft.add(i, 1);
            if let Some((a, b)) = aabb.get(pp[i]).copied() {
                if b == i {
                    let tmp = ft.sum(a..=a);
                    ft.add(a, -(tmp + 1));
                }
            }
        }
        for j in gg[i].iter().copied() {
            rrss[j] = ft.sum(ccdd[j].0..i);
        }
    }
    for rs in rrss {
        println!("{rs}");
    }
}
