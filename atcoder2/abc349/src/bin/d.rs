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
        mut l: usize,
        r: usize,
    };
    let mut rrss = vec![];
    'outer: while l < r {
        for d in (0..=60).rev() {
            let dd = 1usize << d;
            if l + dd <= r && l.is_multiple_of(&dd) {
                rrss.push((l, l + dd));
                l += dd;
                continue 'outer;
            }
        }
    }
    println!("{}", rrss.len());
    for (a, b) in rrss {
        println!("{a} {b}");
    }
}
