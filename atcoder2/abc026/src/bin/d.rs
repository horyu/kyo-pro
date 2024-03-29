#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_complex::ComplexFloat;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    };
    let delta = 1e-7;
    // f(t) = At + Bsin(Ct*PI) = 100
    let f = |t: f64| a * t + b * (c * t * std::f64::consts::PI).sin();
    let mut l = 0.0;
    let mut r = 200.0;
    loop {
        let m = (l + r) / 2.0;
        let diff = f(m) - 100.0;
        if diff.abs() < delta {
            println!("{m}");
            // dbg!(f(m));
            return;
        }
        if 0.0 < diff {
            r = m;
        } else {
            l = m;
        }
    }
}
