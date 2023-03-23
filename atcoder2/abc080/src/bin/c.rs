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
        f: [[usize; 10]; n],
        p: [[isize; 11]; n],
    };
    let mut rs = isize::MIN;
    for vv in (0..10).map(|_| [0usize, 1]).multi_cartesian_product() {
        if vv.iter().all(|&v| v == 0) {
            continue;
        }
        let mut tmp = 0;
        for (ff, pp) in izip!(&f, &p) {
            let i = izip!(ff, &vv).filter(|(f, v)| **f == 1 && **v == 1).count();
            tmp += pp[i];
        }
        rs = rs.max(tmp);
    }
    println!("{rs}");
}
