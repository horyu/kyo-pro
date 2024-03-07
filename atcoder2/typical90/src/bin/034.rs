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
        k: usize,
        aa: [usize; n],
    };
    let mut counter = counter::Counter::<_>::new();
    let mut l = 0;
    let mut rs = 0;
    for r in 0..n {
        counter[&aa[r]] += 1;
        while k < counter.len() {
            counter[&aa[l]] -= 1;
            if counter[&aa[l]] == 0 {
                counter.remove(&aa[l]);
            }
            l += 1;
        }
        rs = rs.max(r - l + 1);
    }
    println!("{rs}");
}
