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
        aabb: [(usize, char); m],
    };
    let mut vv = vec![0; 111];
    for (a, b) in aabb {
        let mut tf = false;
        if b == 'M' {
            if vv[a] == 0 {
                tf = true;
            }
            vv[a] += 1;
        }
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}
