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
        d: usize,
        p: usize,
        mut ff: [usize; n],
    };
    ff.sort_unstable();
    let mut rs = 0;
    while !ff.is_empty() {
        let mut sum = 0;
        for _ in 0..d {
            if let Some(f) = ff.pop() {
                sum += f;
            } else {
                break;
            }
        }
        rs += sum.min(p);
    }
    println!("{rs}");
}
