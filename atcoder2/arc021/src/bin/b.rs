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
        l: usize,
        bb: [usize; l],
    };
    // bi = ai ^ ai+1
    // b: 1 2 3
    // a: 0 1 3
    // ai = bi ^ ai+1
    // ai+1 = bi ^ ai
    let mut aa = vec![0; l];
    for i in 0..(l - 1) {
        aa[i + 1] = bb[i] ^ aa[i];
    }
    for i in 0..l {
        if bb[i] != aa[i] ^ aa[(i + 1) % l] {
            println!("-1");
            return;
        }
    }
    let rs = aa.iter().join("\n");
    println!("{rs}");
}
