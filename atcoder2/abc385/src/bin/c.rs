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
        hh: [usize; n],
    };
    let mut rs = 1;
    for d in 1..=n {
        let mut tmp = 1;
        for dd in 0..d {
            let mut pre = !0;
            for h in hh[dd..].iter().copied().step_by(d) {
                if h == pre {
                    tmp += 1;
                    rs = rs.max(tmp);
                } else {
                    tmp = 1;
                    pre = h;
                }
            }
        }
    }
    println!("{rs}");
}
