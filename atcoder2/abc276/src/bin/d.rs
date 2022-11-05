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
    // b: 2の数, c: 3の数
    let mut bb = vec![0usize; n];
    let mut cc = vec![0usize; n];
    let mut hs = HashSet::new();
    for (i, mut a) in aa.into_iter().enumerate() {
        while a % 2 == 0 {
            bb[i] += 1;
            a /= 2;
        }
        while a % 3 == 0 {
            cc[i] += 1;
            a /= 3;
        }
        hs.insert(a);
    }
    if hs.len() != 1 {
        println!("-1");
        return;
    }
    let mut rs = 0usize;
    let min_b = *bb.iter().min().unwrap();
    rs += bb.iter().fold(0, |acc, &b| acc + b - min_b);
    let min_c = *cc.iter().min().unwrap();
    rs += cc.iter().fold(0, |acc, &c| acc + c - min_c);
    println!("{rs}");
}
