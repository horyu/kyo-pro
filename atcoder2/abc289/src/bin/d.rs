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
        aa: [usize; n],
        m: usize,
        bb: [usize; m],
        x: usize,
    };
    let mut ngs = vec![false; x + 1];
    for b in bb {
        ngs[b] = true;
    }
    let mut vv = vec![false; x + 1];
    vv[0] = true;
    for i in 0..x {
        if vv[i] {
            for a in aa.iter().copied() {
                let j = i + a;
                if j <= x && !ngs[j] {
                    vv[j] = true;
                }
            }
        }
    }
    // eprintln!("{}", vv.iter().join(" "));
    let rs = ["No", "Yes"][vv[x] as usize];
    println!("{rs}");
}
