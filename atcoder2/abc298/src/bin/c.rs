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
        q: usize,
    };
    const MAX: usize = 2 * (1e5 as usize);
    let mut hako = vec![BTreeMap::new(); n];
    let mut card = vec![BTreeSet::new(); MAX];
    for _ in 0..q {
        input! {q: usize};
        if q == 1 {
            input! {i: Usize1, j: Usize1};
            *hako[j].entry(i).or_insert(0usize) += 1;
            card[i].insert(j);
        } else if q == 2 {
            input! {i: Usize1};
            let rs = hako[i]
                .iter()
                .map(|(&k, &v)| vec![k + 1; v].into_iter().join(" "))
                .join(" ");
            println!("{rs}");
        } else {
            input! {i: Usize1};
            let rs = card[i].iter().map(|v| v + 1).join(" ");
            println!("{rs}");
        }
    }
}
