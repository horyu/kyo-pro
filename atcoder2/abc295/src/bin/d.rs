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
        s: Bytes,
    };
    let n = s.len();
    let mut hm = HashMap::new();
    hm.insert(0, 1);
    let mut pre_v = 0usize;
    for (i, b) in s.into_iter().enumerate() {
        let k = b - b'0';
        let v = pre_v ^ (1 << k);
        *hm.entry(v).or_insert(0usize) += 1;
        pre_v = v;
        // eprintln!("{i:2}: {:10b}", v);
    }
    // dbg!(&hm);
    let mut rs = 0usize;
    for v in hm.into_values() {
        rs += v * v.saturating_sub(1) / 2;
    }
    println!("{rs}");
}
