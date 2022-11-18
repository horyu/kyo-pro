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
        q: usize,
        ttxx: [(usize, usize); q],
    };
    // 山札　[0]=一番上
    let mut qq = VecDeque::new();
    for (t, x) in ttxx {
        match t {
            1 => {
                qq.push_front(x);
            }
            2 => {
                qq.push_back(x);
            }
            _ => {
                println!("{}", qq[x - 1]);
            }
        }
    }
}
