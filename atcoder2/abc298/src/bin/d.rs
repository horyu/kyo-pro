#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        q: usize,
    };
    let mut m = ModInt998244353::new(1);
    let mut vq = VecDeque::from([1]);
    for _ in 0..q {
        input! {q: usize};
        if q == 1 {
            input! {x: usize};
            vq.push_back(x);
            m = m * 10 + x;
        } else if q == 2 {
            let y = vq.pop_front().unwrap();
            let sub = ModInt998244353::new(10).pow(vq.len() as u64) * y;
            m -= sub;
        } else {
            println!("{m}");
        }
    }
}
