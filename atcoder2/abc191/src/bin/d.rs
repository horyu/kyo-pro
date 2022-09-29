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
        x: String,
        y: String,
        r: String,
    };
    // x,y: 符号と整数部は無視する
    let read = |s: &str, is_r: bool| -> isize {
        let vv = s.split('.').collect_vec();
        let mut num = if is_r { vv[0].parse::<isize>().unwrap() * 10000 } else { 0 };
        if let Some(&v) = vv.get(1) {
            num += v.parse::<isize>().unwrap()
                * 10isize.pow(4 - v.len() as u32);
        }
        num
    };
    let x = read(&x, false);
    let y = read(&y, false);
    let r = read(&r, true);
    dbg!(x, y, r);
    let rr = r.pow(2);
    let mut rs = 0;
    for i in (-10000)..=10000 {
        let a = i + x;
        let aa = a.pow(2);
        if aa <= rr {
            let bb = rr - aa;
            // aa + bb = rr
            let b = bb.sqrt();
            // b = j + y;

        }
    }
    println!("{rs}");
}
