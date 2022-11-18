#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::consts::PI,
};

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        ee: [f64; q],
    };
    let z = 0.0;
    let xx = 0.0;
    for e in ee {
        // e 0 t/2 t
        //   0 PI  2PI
        let theta = 2.0 * PI * e / t;
        let yy = -(l / 2.0) * theta.sin();
        let zz = (l / 2.0) * (1.0 - theta.cos());

        let xy = ((x - xx).powi(2) + (y - yy).powi(2)).sqrt();
        let tan = zz / xy;
        let rs = tan.atan() * 180.0 / PI;
        println!("{rs}");
    }
}
