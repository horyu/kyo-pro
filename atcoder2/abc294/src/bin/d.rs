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
    };
    let mut mada = (1..=n).collect::<BTreeSet<_>>();
    let mut taiki = BTreeSet::new();
    for _ in 0..q {
        input! {qt: usize};
        match qt {
            1 => {
                if let Some(next) = mada.pop_first() {
                    taiki.insert(next);
                }
            }
            2 => {
                input! {x: usize}
                taiki.remove(&x);
            }
            _ => {
                if let Some(next) = taiki.first() {
                    println!("{next}");
                }
            }
        }
    }
}
