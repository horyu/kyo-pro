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
        s: Chars,
        t: String,
    };
    for w in 1..s.len() {
        for c in 0..w {
            let string = s
                .iter()
                .copied()
                .chunks(w)
                .into_iter()
                .filter_map(|mut sscc| sscc.nth(c))
                .collect::<String>();
            if string == t {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
