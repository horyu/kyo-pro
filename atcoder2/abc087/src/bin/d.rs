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
        llrrdd: [(Usize1, Usize1, isize); m],
    };
    let mut dsu = ac_library::Dsu::new(n);
    let mut xxdd = vec![vec![]; n];
    for &(l, r, d) in &llrrdd {
        dsu.merge(l, r);
        xxdd[l].push((r, d));
        xxdd[r].push((l, -d));
    }
    let mut vv = vec![0isize; n];
    let mut pushed = vec![false; n];
    for ii in dsu.groups() {
        let mut qq = VecDeque::new();
        qq.push_back(ii[0]);
        pushed[ii[0]] = true;
        while let Some(q) = qq.pop_front() {
            for &(x, d) in &xxdd[q] {
                vv[x] = vv[q] + d;
                if !pushed[x] {
                    pushed[x] = true;
                    qq.push_back(x);
                }
            }
        }
    }
    for &(l, r, d) in &llrrdd {
        if vv[l] + d != vv[r] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
