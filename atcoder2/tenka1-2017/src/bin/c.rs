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
    const MAX: isize = 3500;
    // 4/N=1/a+1/b+1/c
    // 4abc = N(bc+ac+ab)
    // c(4ab-Nb-Na) = Nab
    // c = Nab/(4ab-Nb-Na)
    for a in 1..=MAX {
        for b in 1..=MAX {
            let c = (n * a * b)
                .checked_div(4 * a * b - n * b - n * a)
                .unwrap_or_default();
            if 0 < c && 4 * a * b * c == n * (b * c + a * c + a * b) {
                println!("{a} {b} {c}");
                return;
            }
        }
    }
}
