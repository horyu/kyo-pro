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
        x: String,
        y: String,
        r: String,
    };
    let read = |s: &str, is_r: bool| -> isize {
        let num = s.parse::<f64>().unwrap();
        if is_r {
            (num * 10000.0).round() as isize
        } else {
            (num.fract().abs() * 10000.0).round() as isize
        }
    };
    let x = read(&x, false);
    let y = read(&y, false);
    let r = read(&r, true);
    let rr = r.pow(2);
    let mut rs = 0;
    for i in (-100000)..=100000 {
        // (X - x)^2 + (Y - y)^2 <= r^2
        // -sqrt() + y <= Y <= +sqrt() + y
        let a = i * 10000 + x;
        let aa = a.pow(2);
        if aa <= rr {
            let bb = rr - aa;
            // aa + bb = rr
            let b = bb.sqrt();
            // b = j - y;
            let ju = b + y;
            let jd = -b + y;
            if jd < 0 {
                rs += (jd / 10000).abs() + ju / 10000 + 1;
            } else if ju == jd {
                if ju % 10000 == 0 {
                    rs += 1;
                }
            } else {
                rs += 1 + ju / 10000 - jd.div_ceil(10000);
            }
            // eprintln!("{i} {rs} {jd}..{ju}");
        }
    }
    println!("{rs}");
}
