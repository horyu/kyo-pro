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
        q: usize,
    };
    let mut poss = (0..n).collect_vec();
    let mut cc = vec![1; n];
    let mut rs = 0;
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {p: Usize1, h: Usize1};
            let from = poss[p];
            cc[from] -= 1;
            if cc[from] == 1 {
                rs -= 1;
            }
            let to = h;
            poss[p] = to;
            cc[to] += 1;
            if cc[to] == 2 {
                rs += 1;
            }
            continue;
        }
        println!("{rs}");
    }
}
