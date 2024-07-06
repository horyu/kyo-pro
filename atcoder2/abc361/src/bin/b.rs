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
        // q: [(usize, usize, usize); 4],
        q: [[usize; 3]; 4],
    };
    let tf = (0..3).all(|pos| {
        let (a, b) = (q[0][pos], q[1][pos]);
        let (c, d) = (q[2][pos], q[3][pos]);
        if a <= c {
            (a..b).contains(&c)
        } else {
            (c..d).contains(&a)
        }
    });
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
