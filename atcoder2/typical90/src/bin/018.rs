#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        ee: [f64; q],
    };
    for e in ee {
        let theta = e / t * std::f64::consts::TAU;
        let pos = (0, -l / 2.0 * theta.sin(), (l - l * theta.cos()) / 2.0);
        let dr = (x.powi(2) + (y - pos.1).powi(2)).sqrt();
        let rs = (pos.2 / dr).atan() * std::f64::consts::FRAC_1_PI * 180.0;
        println!("{rs}");
    }
}
