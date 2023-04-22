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
        s: Chars,
    };
    let mut rs = -1;
    for (x, y) in s
        .into_iter()
        .group_by(|&c| c)
        .into_iter()
        .map(|cg| (cg.0, cg.1.into_iter().count() as isize))
        .tuple_windows()
    {
        if x.0 == 'o' {
            rs = rs.max(x.1);
        } else {
            rs = rs.max(y.1);
        }
    }
    println!("{rs}");
}
