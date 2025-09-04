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
        m: usize,
        aa: [usize; m],
    };
    let mut btm: BTreeMap<i32, BTreeSet<usize>> = BTreeMap::new();
    let mut cc = vec![0; n + 1];
    for a in aa {
        if 0 < cc[a] {
            if let Some(set) = btm.get_mut(&cc[a]) {
                set.remove(&a);
                if set.is_empty() {
                    btm.remove(&cc[a]);
                }
            }
        }
        cc[a] += 1;
        btm.entry(cc[a]).or_default().insert(a);
        if let Some(e) = btm.last_entry() {
            let rs = e.get().first().unwrap();
            println!("{rs}");
        }
    }
}
