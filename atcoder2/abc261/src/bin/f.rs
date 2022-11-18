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
        cc: [usize; n],
        xx: [usize; n],
    };
    let mut rs = 0;
    let xtoi: HashMap<usize, usize> = xx
        .iter()
        .unique()
        .sorted()
        .enumerate()
        .map(|(i, &x)| (x, i + 1))
        .collect();
    // 色を考えない反転数
    {
        let mut b = Bit::new(xtoi.len());
        for (i, &x) in xx.iter().enumerate() {
            let x = *xtoi.get(&x).unwrap();
            rs += i - b.sum(x);
            b.add(x);
        }
    }
    // 色ごとの反転数を引く
    for ii in (0..n)
        .into_iter()
        .into_group_map_by(|&i| cc[i])
        .into_values()
    {
        let vv = ii.into_iter().map(|i| xx[i]).collect_vec();
        let vtoj: HashMap<usize, usize> = vv
            .iter()
            .unique()
            .sorted()
            .enumerate()
            .map(|(j, &v)| (v, j + 1))
            .collect();
        let mut b = Bit::new(vtoj.len());
        for (j, v) in vv.into_iter().enumerate() {
            let v = *vtoj.get(&v).unwrap();
            rs -= j - b.sum(v);
            b.add(v);
        }
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
