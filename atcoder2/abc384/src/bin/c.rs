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
        xx: [isize; 5],
    };
    let mut mm = btreemultimap::BTreeMultiMap::new();
    for bits in 1u32..(1 << 5) {
        let mut sum = 0;
        let mut s = String::new();
        for i in 0..5 {
            if bits & (1 << i) != 0 {
                sum += xx[i];
                s.push((b'A' + i as u8) as char);
            }
        }
        mm.insert(sum, s);
    }
    for (sum, mut ss) in mm.into_iter().rev() {
        ss.sort_unstable();
        // eprintln!("{sum} {ss:?}");
        for s in ss {
            println!("{s}");
        }
    }
}
