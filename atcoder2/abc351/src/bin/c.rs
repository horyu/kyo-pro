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
    let mut vv = vec![];
    for a in aa {
        vv.push(a);
        loop {
            let len = vv.len();
            if 2 <= len && vv[len - 2] == vv[len - 1] {
                vv.pop();
                vv[len - 2] += 1;
                continue;
            }
            break;
        }
    }
    let rs = vv.len();
    println!("{rs}");
}
