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
        aa: [Usize1; n],
    };
    let mut counter = counter::Counter::<_>::new();
    for a in aa.iter() {
        counter[a] += 1;
    }
    if counter.len() < m {
        println!("0");
        return;
    }
    let mut rs = 0;
    for a in aa.iter().copied().rev() {
        rs += 1;
        counter[&a] -= 1;
        if counter[&a] == 0 {
            println!("{rs}");
            return;
        }
    }
}
