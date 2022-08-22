#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_complex::ComplexFloat;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    };
    use std::f64::consts::PI;
    let delta = 1e-7;
    // f(t) = At + Bsin(Ct*PI) = 100
    let f = |t: f64| a * t + b * (c * t * PI).sin();
    let bserach = |mut l: f64, mut r: f64| -> f64 {
        assert!(f(l) <= f(r));
        loop {
            let m = (l + r) / 2.0;
            let diff = f(m) - 100.0;
            if diff.abs() < delta {
                return m;
            }
            if 0.0 < diff {
                r = m;
            } else {
                l = m;
            }
        }
    };
    let rs = bserach(0.0, 200.0);
    println!("{rs}");
}
