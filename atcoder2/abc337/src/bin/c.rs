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
        aa: [isize; n],
    };
    let mut cc = vec![!0; n];
    let mut first = 0;
    for (i, a) in aa.iter().copied().enumerate() {
        if a == -1 {
            first = i;
            continue;
        }
        cc[a as usize - 1] = i;
    }
    let mut rrss = vec![first];
    for _ in 1..n {
        let last = *rrss.last().unwrap();
        let next = cc[last];
        rrss.push(next);
    }
    let rs = rrss.iter().map(|x| x + 1).join(" ");
    println!("{rs}");
}
