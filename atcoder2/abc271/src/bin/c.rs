#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let mut bts = BTreeSet::new();
    let mut over = 0;
    for a in aa {
        if !bts.insert(a) {
            over += 1;
        }
    }
    let mut rs = 0;
    'outer: for i in 1.. {
        if bts.contains(&i) {
            rs = i;
            continue;
        }
        for _ in 0..(2i32.saturating_sub(over)) {
            if let Some(&max) = bts.iter().max() {
                if i < max {
                    bts.remove(&max);
                    over += 1;
                } else {
                    break 'outer;
                }
            }
        }
        if 2 <= over {
            over -= 2;
            rs = i;
        } else {
            break;
        }
    }
    println!("{rs}");
}
