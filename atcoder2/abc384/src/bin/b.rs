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
        mut r: isize,
        ddaa: [(usize, isize); n],
    };
    for (d, a) in ddaa {
        if d == 1 {
            if (1600..2800).contains(&r) {
                r += a;
            }
        } else {
            if (1200..2400).contains(&r) {
                r += a;
            }
        }
    }
    let rs = r;
    println!("{rs}");
}
