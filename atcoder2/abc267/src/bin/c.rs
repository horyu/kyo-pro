#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
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
        aa: [isize; n],
    };
    if m == 1 {
        let rs = aa.into_iter().max().unwrap();
        println!("{rs}");
        return;
    }
    let mut v = (0..m).fold(0, |acc, i| acc + (i + 1) as isize * aa[i]);
    let mut rs = v;
    // dbg!(v);
    let mut sub = aa[..m].iter().sum::<isize>();
    for i in 0..(n - m) {
        v -= sub - m as isize * aa[i + m];
        rs = rs.max(v);
        // dbg!(v);
        // eprintln!("{sub}->{}", sub - aa[i] + aa[i + m]);
        sub = sub - aa[i] + aa[i + m];
    }
    println!("{rs}");
}
