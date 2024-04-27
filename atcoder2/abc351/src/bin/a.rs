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
        aa: [usize; 9],
        bb: [usize; 8]
    };
    let a_sum = aa.iter().sum::<usize>();
    let b_sum = bb.iter().sum::<usize>();
    let rs = (a_sum + 1).saturating_sub(b_sum);
    println!("{rs}");
}
