#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        s: Chars,
    };
    let cnts = s.iter().copied().counts();
    let mut counter = counter::Counter::<_>::new();
    let mut rs = 0;
    for c in s {
        for (&k, &l) in &counter {
            let r = cnts[&k] - l - usize::from(c == k);
            rs += l * r;
        }
        counter[&c] += 1;
    }
    println!("{rs}");
}
