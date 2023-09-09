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
        x: usize,
        y: usize,
        pptt: [(usize, usize); n - 1],
        q: usize,
        qq: [usize; q],
    };
    let mut ww = vec![];
    for mut i in 0..840 {
        for (p, t) in pptt.iter().copied() {
            i = i.next_multiple_of(&p) + t;
        }
        ww.push(i);
    }
    for q in qq {
        let rs = (x + q) / 840 * 840 + ww[(x + q) % 840] + y;
        println!("{rs}");
    }
}
