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
        aa: [usize; n],
    };
    let mut btm = BTreeMap::new();
    for a in aa.into_iter().rev() {
        let mut should_remove = None;
        if let Some((k, e)) = btm.range_mut((a + 1)..).next() {
            if *e == 1 {
                should_remove = Some(*k);
            } else {
                *e -= 1;
            }
        }
        if let Some(remove_key) = should_remove {
            btm.remove(&remove_key);
        }

        *btm.entry(a).or_insert(0) += 1usize;
    }
    let rs = btm.into_values().sum::<usize>();
    println!("{rs}");
}
