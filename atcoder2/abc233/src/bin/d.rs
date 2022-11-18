#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: isize,
        aa: [isize; n],
    };
    let mut hm = HashMap::new();
    let mut sum = 0;
    for (i, &a) in aa.iter().enumerate() {
        sum += a;
        *hm.entry(sum).or_insert(0) += 1;
    }
    let mut rs = 0usize;
    let mut crr = 0;
    for a in aa {
        // eprintln!("{a:3} {:3} {:3} {:?}", k + crr, hm.get(&(k + crr)).unwrap_or(&0), &hm);
        rs += hm.get(&(k + crr)).unwrap_or(&0);
        crr += a;
        *hm.get_mut(&crr).unwrap() -= 1;
    }
    println!("{rs}");
}
