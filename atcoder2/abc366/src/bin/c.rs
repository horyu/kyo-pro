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
        q: usize,
    };
    let mut rs = 0;
    let mut vv = vec![0; 1e6 as usize + 1];
    for _ in 0..q {
        input! {t: usize};
        if t == 3 {
            println!("{rs}");
            continue;
        }
        input! {x: usize};
        if t == 1 {
            if vv[x] == 0 {
                rs += 1;
            }
            vv[x] += 1;
        } else {
            vv[x] -= 1;
            if vv[x] == 0 {
                rs -= 1;
            }
        }
    }
}