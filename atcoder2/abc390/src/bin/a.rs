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
        mut aa: [Usize1; 5],
    };
    for i in 0..4 {
        aa.swap(i, i + 1);
        if aa == [0, 1, 2, 3, 4] {
            println!("Yes");
            return;
        }
        aa.swap(i, i + 1);
    }
    println!("No");
}
