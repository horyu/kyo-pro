#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut aa: [isize; n],
    };
    for rs in 1.. {
        aa.sort_unstable();
        aa.reverse();
        aa[0] -= 1;
        aa[1] -= 1;
        if aa.iter().filter(|a| a.is_positive()).at_most_one().is_ok() {
            println!("{rs}");
            return;
        }
    }
}
