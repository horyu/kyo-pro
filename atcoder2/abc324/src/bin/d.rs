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
        s: Bytes,
    };
    let mut scc = [0; 10];
    for &c in &s {
        scc[(c - b'0') as usize] += 1;
    }
    if scc[0] == n {
        println!("1");
        return;
    }
    let mut rs = 0usize;
    for i in 1isize.. {
        let mut ii = i.pow(2);
        if n < ii.to_string().len() {
            break;
        }
        let mut icc = [0; 10];
        while 0 < ii {
            icc[(ii % 10) as usize] += 1;
            ii /= 10;
        }
        if (1..=9).all(|i| scc[i] == icc[i]) {
            rs += 1;
        }
    }
    println!("{rs}");
}
