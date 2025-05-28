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
        q: usize,
    };
    let mut hhss = vec![HashSet::new(); n + 1];
    for _ in 0..q {
        input! {t: usize};
        match t {
            1 => {
                input! {x: usize, y: usize};
                hhss[x].insert(y);
            }
            2 => {
                input! {x: usize};
                hhss[x].insert(0);
            }
            // 3
            _ => {
                input! {x: usize, y: usize};
                let tf = hhss[x].contains(&0) || hhss[x].contains(&y);
                let rs = ["No", "Yes"][tf as usize];
                println!("{rs}");
            }
        }
    }
}
