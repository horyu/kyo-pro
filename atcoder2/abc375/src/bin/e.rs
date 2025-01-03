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
        aabb: [(Usize1, usize); n],
    };
    let mut sum = 0;
    let mut vvv = vec![vec![]; 3];
    for (a, b) in aabb {
        sum += b;
        vvv[a].push(b);
    }
    if sum % 3 != 0 {
        println!("-1");
        return;
    }
    let target = sum / 3;
    dbg!(sum, target);
    // println!("{rs}");
}
