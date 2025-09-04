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
        l: usize,
        aa: [usize; n],
    };
    let mut bh = BinaryHeap::new();
    bh.push(l);
    for a in aa {
        let x = bh.pop().unwrap();
        if a == 2 && x < 2 {
            println!("No");
            return;
        }
        match x.cmp(&(1 + a)) {
            Ordering::Less => (),
            Ordering::Equal => {
                bh.push(1);
            },
            Ordering::Greater => {
                bh.push(1);
                bh.push(x - (1 + a));
            },
        }
    }
    println!("Yes");
}
