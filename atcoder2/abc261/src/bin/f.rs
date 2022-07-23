#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        cc: [usize; n],
        xx: [usize; n],
    };
    let ctoi: HashMap<usize, usize> = cc
        .iter()
        .unique()
        .enumerate()
        .map(|(i, &c)| (c, i + 1))
        .collect();
    let mut bb = vec![Bit::new(300000); ctoi.len() + 1];
    let mut rs = 0;
    for (i, c, x) in izip!(0..n, cc, xx) {
        let c = *ctoi.get(&c).unwrap();
        eprintln!("{rs}({i}, {c}) {} {}", bb[c].sum(x), bb[0].sum(x));
        rs += bb[0].sum(x) - bb[c].sum(x);
        bb[0].add(x);
        bb[c].add(x);
    }
    println!("{rs}");
}

#[derive(Clone)]
struct Bit {
    bit: Vec<usize>,
    n: usize,
}
impl Bit {
    fn new(n: usize) -> Self {
        Self {
            bit: vec![0; n + 1],
            n,
        }
    }

    fn add(&mut self, a: usize) {
        let mut x = a as isize;
        while x <= (self.n as isize) {
            self.bit[x as usize] += 1;
            x += x & (-x);
        }
    }

    fn sum(&mut self, a: usize) -> usize {
        let mut rs = 0;
        let mut x = a as isize;
        while 0 < x {
            rs += self.bit[x as usize];
            x -= x & (-x);
        }
        rs
    }
}
