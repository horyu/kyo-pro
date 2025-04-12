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
        q: usize,
    };
    let mut vv = vec![0; 100];
    for _ in 0..q {
        input! {t: i32};
        if t == 1 {
            input! {x: i32};
            vv.push(x);
        } else {
            let rs = vv.pop().unwrap();
            println!("{rs}");
        }
    }
}
