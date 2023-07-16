#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut pp = vec![];
    let mut hhss: Vec<HashSet<usize>> = vec![];
    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            ff: [usize; c],
        };
        pp.push(p);
        hhss.push(ff.into_iter().collect());
    }
    for i in 0..n {
        for j in 0..n {
            if pp[j] <= pp[i]
                && hhss[i].is_subset(&hhss[j])
                && (pp[j] < pp[i] || hhss[i].len() < hhss[j].len())
            {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
