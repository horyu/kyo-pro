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
        bb: [usize; n],
    };
    let mut aabb = izip!(aa, bb).collect_vec();
    // https://qiita.com/python_walker/items/d1e2be789f6e7a0851e5
    let lis = |xx: &[usize]| -> usize {
        let mut vv = vec![xx[0]];
        for (i, &x) in xx.iter().enumerate() {
            if *vv.last().unwrap() < x {
                vv.push(x);
            } else {
                let i = vv.partition_point(|&v| v < x);
                vv[i] = x;
            }
        }
        vv.len()
    };
    aabb.sort_unstable_by_key(|ab| ab.0);
    let rs = lis(aabb.iter().map(|ab| ab.0).collect_vec().as_ref())
        + lis(aabb.iter().map(|ab| ab.1).collect_vec().as_ref());
    println!("{rs}");
}
