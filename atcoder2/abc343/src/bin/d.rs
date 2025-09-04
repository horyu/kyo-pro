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
        t: usize,
        aabb: [(Usize1, usize); t],
    };
    let mut cc = vec![0; n];
    let mut counter = counter::Counter::<_>::new();
    counter[&0] = n;
    for (a, b) in aabb {
        let pre_c = cc[a];
        counter[&pre_c] -= 1;
        if counter[&pre_c] == 0 {
            counter.remove(&pre_c);
        }

        cc[a] += b;
        counter[&cc[a]] += 1;

        let rs = counter.len();
        println!("{rs}");
    }
}
