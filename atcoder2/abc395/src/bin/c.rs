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
        aa: [usize; n],
    };
    let mut mm = multimap::MultiMap::new();
    for (i, a) in aa.into_iter().enumerate() {
        mm.insert(a, i);
    }
    let rs = mm
        .into_iter()
        .flat_map(|(_, ii)| {
            ii.into_iter()
                .tuple_windows()
                .map(|(i, j)| (1 + j - i) as isize)
        })
        .min()
        .unwrap_or(-1);
    println!("{rs}");
}
