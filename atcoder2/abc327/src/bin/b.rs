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
        b: usize,
    };
    for a in 1usize.. {
        let mut tmp = a;
        for _ in 1..a {
            tmp = tmp.saturating_mul(a);
        }
        match tmp.cmp(&b) {
            Ordering::Equal => {
                println!("{a}");
                return;
            }
            Ordering::Greater => {
                break;
            }
            _ => {}
        }
    }
    println!("-1");
}
