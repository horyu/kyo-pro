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
        mut aa: [usize; n],
    };
    let mut bb = aa.split_off(n / 2);
    let mut rs = 0;
    while let Some(b) = bb.pop() {
        while let Some(a) = aa.pop() {
            if a * 2 <= b {
                rs += 1;
                break;
            }
        }
    }
    println!("{rs}");
}
