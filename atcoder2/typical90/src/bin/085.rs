#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        k: usize,
    };
    // 2 * 12=> 2 * [1*12,2*6,3*4]
    // 4 * 6 => 4 * [1*6,2*3]
    let mut rs = 0;
    for a in 1..=(k.sqrt()) {
        let bc = k / a;
        if a * bc == k {
            for b in a..=(bc.sqrt()) {
                let c = bc / b;
                if b * c == bc && b <= c {
                    // eprintln!("{a} {b} {c}");
                    rs += 1;
                }
            }
        }
    }
    println!("{rs}");
}
