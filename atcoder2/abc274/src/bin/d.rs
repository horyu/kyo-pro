#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::format,
};

fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
        mut aa: [isize; n],
    };
    let mut ii = HashSet::new();
    ii.insert(aa[0]);
    for &a in aa[2..].iter().step_by(2) {
        let mut new_ii = HashSet::new();
        for i in ii {
            new_ii.insert(i + a);
            new_ii.insert(i - a);
        }
        ii = new_ii;
    }
    let mut jj = HashSet::new();
    jj.insert(0);
    for &a in aa[1..].iter().step_by(2) {
        let mut new_jj = HashSet::new();
        for j in jj {
            new_jj.insert(j + a);
            new_jj.insert(j - a);
        }
        jj = new_jj;
    }
    let tf = ii.contains(&x) && jj.contains(&y);
    let rs = if tf { "Yes" } else { "No" };
    println!("{rs}");
}
