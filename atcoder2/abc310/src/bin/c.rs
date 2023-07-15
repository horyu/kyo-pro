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
        mut ss: [Chars; n],
    };
    let mut hs = HashSet::new();
    let mut rs = 0;
    for s in ss {
        let rev = s.iter().copied().rev().collect_vec();
        if s == rev {
            if hs.insert(s) {
                rs += 1;
            }
        } else {
            if hs.insert(s) && hs.insert(rev) {
                rs += 1;
            }
        }
    }
    println!("{rs}");
}
