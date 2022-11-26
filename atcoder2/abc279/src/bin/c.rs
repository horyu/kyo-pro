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
        h: usize,
        w: usize,
        ss: [Chars; h],
        tt: [Chars; h],
    };
    let mut hm = HashMap::new();
    for j in 0..w {
        let mut vv = vec![];
        let mut ww = vec![];
        for i in 0..h {
            vv.push(ss[i][j]);
            ww.push(tt[i][j]);
        }
        *hm.entry(vv).or_insert(0i32) += 1;
        *hm.entry(ww).or_insert(0i32) -= 1;
    }
    let tf = hm.into_values().all(|v| v == 0);
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
