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
        n: isize,
    };
    // 4/n = 1/x + 1/y + 1/z
    // 4xyz = n(xy + yz + zx)
    // z(4xy - nx - ny) = nxy
    // z = nxy/(4xy - nx - ny)
    for x in 1..=3500 {
        for y in 1..=3500 {
            if let Some(z) = (n * x * y).checked_div(4 * x * y - n * x - n * y) {
                if (1..=3500).contains(&z) && 4 * x * y * z == n * (x * y + y * z + z * x) {
                    println!("{x} {y} {z}");
                    return;
                }
            }
        }
    }
}
