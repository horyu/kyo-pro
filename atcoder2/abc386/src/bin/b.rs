#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: Chars,
    };
    let rs = s
        .into_iter()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, g)| {
            if c == '0' {
                g.count().div_ceil(2)
            } else {
                g.count()
            }
        })
        .sum::<usize>();
    println!("{rs}");
}
