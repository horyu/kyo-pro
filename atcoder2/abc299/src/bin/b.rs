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
        t: Usize1,
        cc: [Usize1; n],
        rr: [usize; n],
    };
    let t = if cc.contains(&t) { t } else { cc[0] };
    let i = izip!(cc, rr)
        .enumerate()
        .filter(|icr| icr.1 .0 == t)
        .max_by_key(|icr| icr.1 .1)
        .unwrap()
        .0;
    let rs = i + 1;
    println!("{rs}");
}
