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
        q: usize,
        iixx: [(Usize1, Usize1); q],
    };
    let mut cc = vec![0; n];
    for (i, x) in iixx {
        match i {
            0 => {
                cc[x] += 1;
            }
            1 => {
                cc[x] += 2;
            }
            _ => {
                let rs = ["No", "Yes"][(2 <= cc[x]) as usize];
                println!("{rs}");
            }
        }
    }
}
