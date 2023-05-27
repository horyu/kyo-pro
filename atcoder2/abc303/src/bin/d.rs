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
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    };
    // Off/On
    let mut vv = [0, 1 << 60];
    for c in s {
        let mut ww = [0; 2];
        if c == 'a' {
            ww[0] = (vv[0] + x).min(z + vv[1] + x);
            ww[1] = (vv[0] + z + y).min(vv[1] + y);
        } else {
            ww[0] = (vv[0] + y).min(z + vv[1] + y);
            ww[1] = (z + vv[0] + x).min(vv[1] + x);
        }
        vv = ww;
        // eprintln!("{vv:?}");
    }
    let rs = vv.into_iter().min().unwrap();
    println!("{rs}");
}
