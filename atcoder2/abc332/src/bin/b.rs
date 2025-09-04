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
        k: usize,
        g: usize,
        m: usize,
    };
    let mut grass = 0;
    let mut mugcup = 0;
    for _ in 0..k {
        if grass == g {
            grass = 0;
        } else if mugcup == 0 {
            mugcup = m;
        } else {
            let x = mugcup.min(g - grass);
            grass += x;
            mugcup -= x;
        }
    }
    println!("{grass} {mugcup}");
}
