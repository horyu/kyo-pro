#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    for y in ((m.is_odd() as usize)..=n).step_by(2) {
        //  x +  z = n - y  = p
        // 2x + 4z = m - 3y = q
        //      2z = q - 2p
        let p = n - y;
        let q = m.saturating_sub(3 * y);
        let z = (q.saturating_sub(2 * p)) / 2;
        let x = n.saturating_sub(y + z);
        if x + y + z == n && 2 * x + 3 * y + 4 * z == m {
            println!("{x} {y} {z}");
            return;
        }
    }
    println!("-1 -1 -1");
}
