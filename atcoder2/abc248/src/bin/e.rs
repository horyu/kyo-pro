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
        k: usize,
        xxyy: [(i128, i128); n],
    };
    if k == 1 {
        println!("Infinity");
        return;
    }
    let mut rs = 0usize;
    for vv in xxyy.iter().copied().into_group_map_by(|xy| xy.0).values() {
        rs += (k <= vv.len()) as usize;
    }
    for vv in xxyy.iter().copied().into_group_map_by(|xy| xy.1).values() {
        rs += (k <= vv.len()) as usize;
    }
    // 2点 1
    // 3点 3
    // 4点 6
    // n点 n*(n-1)/2
    let mut counter = counter::Counter::<(i128, i128, i128)>::new();
    for ((px, py), (qx, qy)) in xxyy.iter().copied().tuple_combinations() {
        let dx = px - qx;
        let dy = py - qy;
        if dx != 0 && dy != 0 {
            let sign = dy.signum();
            let gcd = dx.gcd(&dy);
            let kx = sign * dx / gcd;
            let ky = sign * dy / gcd;
            // X - x = k(Y - y)
            // Y=0のとき X0 = -k*y + x
            // X0 = (-kx * y + ky * x) / ky
            counter[&(kx, ky, -kx * py + ky * px)] += 1;
            // eprintln!("{px} {py} {qx} {qy} {}", -kx * py + ky * px);
        }
    }
    for c in counter.values().copied() {
        if k * (k - 1) / 2 <= c {
            rs += 1;
        }
    }

    println!("{rs}");
}
