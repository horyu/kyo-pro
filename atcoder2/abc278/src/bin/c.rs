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
        q: usize,
        ttaabb: [(usize, usize, usize); q],
    };
    let mut hs = HashSet::new();
    for (t, a, b) in ttaabb {
        match t {
            1 => {
                hs.insert((a, b));
            }
            2 => {
                hs.remove(&(a, b));
            }
            _ => {
                let rs = if hs.contains(&(a, b)) && hs.contains(&(b, a)) {
                    "Yes"
                } else {
                    "No"
                };
                println!("{rs}");
            }
        }
    }
}
