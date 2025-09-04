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
        mut aa: [usize; n],
    };
    'outer: loop {
        let len = aa.len();
        for i in 0..(len - 1) {
            let l = aa[i];
            let r = aa[i + 1];
            if 1 == l.abs_diff(r) {
                continue;
            }
            if l < r {
                for k in ((l + 1)..r).rev() {
                    aa.insert(i + 1, k)
                }
            } else {
                for k in (r + 1)..l {
                    aa.insert(i + 1, k)
                }
            }
            continue 'outer;
        }
        println!("{}", aa.iter().join(" "));
        return;
    }
}
