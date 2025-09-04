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
        k: usize,
        s: Bytes,
    };
    let mut b_to_ii = vec![BTreeSet::new(); 256];
    for (i, b) in s.iter().copied().enumerate() {
        b_to_ii[b as usize].insert(i);
    }
    let mut rs = String::new();
    let mut l = 0;
    for j in 0..k {
        for b in b'a'..=b'z' {
            if let Some(i) = b_to_ii[b as usize].range(l..=(n - k + j)).next().copied() {
                rs.push(b as char);
                l = i + 1;
                break;
            }
        }
    }
    println!("{rs}");
}
