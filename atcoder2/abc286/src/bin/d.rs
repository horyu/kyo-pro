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
        x: usize,
        aabb: [(usize, usize); n],
    };
    let mut hs = HashSet::<usize>::new();
    for (a, b) in aabb {
        let mut new_hs = hs.clone();
        for bb in 1..=b {
            for k in hs.iter().copied() {
                let kk = k + a * bb;
                if kk <= x {
                    new_hs.insert(kk);
                }
            }
            let kk = a * bb;
            if kk <= x {
                new_hs.insert(kk);
            }
        }
        hs = new_hs;
    }
    let rs = ["No", "Yes"][hs.contains(&x) as usize];
    println!("{rs}");
}
