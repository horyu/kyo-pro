#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        r: isize,
    };
    let mut rs = 0;
    // -r+0.5 ~ r-0.5
    for i in -r..(r - 1) {
        let i = i as f64;
        let xl = i + 0.5;
        let xr = i + 1.5;
        let x_max = xl.abs().max(xr.abs());
        // xx + yy = rr => y = sqrt(rr - xx)
        let y = (r.pow(2) as f64 - x_max.powi(2)).sqrt();
        if y < 0.5 {
            continue;
        }
        rs += 1 + 2 * (y - 0.5).floor() as usize;
    }
    println!("{rs}");
}
