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
        n: usize,
        aabb: [(Usize1, Usize1); n],
    };
    let mut dsu = ac_library::Dsu::new(4e5 as usize);
    for (a, b) in aabb.iter().copied() {
        dsu.merge(a, b);
    }
    let mut counter = counter::Counter::<usize>::new();
    let mut leader2size = HashMap::new();
    for (i, (a, b)) in aabb.iter().copied().enumerate() {
        counter[&dsu.leader(a)] += 1;
        leader2size.insert(dsu.leader(a), dsu.size(a));
    }
    let mut rs = 0;
    for (leader, size) in leader2size {
        let count = counter[&leader];
        if size <= count {
            rs += size;
        } else {
            rs += size - 1;
        }
    }
    println!("{rs}");
}
