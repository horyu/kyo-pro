#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        x: usize,
        aa: [usize; n],
    };
    let mut rs = 0usize;
    let mut bb = vec![];
    for a in aa {
        if x < a {
            rs += a - x;
            bb.push(x);
        } else {
            bb.push(a);
        }
    }
    for i in 1..n {
        let wa = bb[i - 1] + bb[i];
        if x < wa {
            let diff = wa - x;
            bb[i] -= diff;
            rs += diff;
        }
    }
    println!("{rs}");
}
