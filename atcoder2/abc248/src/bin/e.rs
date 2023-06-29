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
    let kk = xxyy
        .iter()
        .copied()
        .tuple_combinations()
        .filter_map(|((px, py), (qx, qy))| {
            let dx = px - qx;
            let dy = py - qy;
            (dx != 0 && dy != 0).then_some({
                let sign = dy.signum();
                let gcd = dx.gcd(&dy);
                (sign * dx / gcd, sign * dy / gcd)
            })
        })
        .unique()
        .collect_vec();
    for (kx, ky) in kk.iter().copied() {
        let mut counter = counter::Counter::<i128, usize>::new();
        for (x, y) in xxyy.iter().copied() {
            // X - x = k(Y - y)
            // Y=0のとき X0 = -k*y + x
            // X0 = (-kx * y + ky * x) / ky
            let tmp = -kx * y + ky * x;
            counter[&tmp] += 1;
        }
        for c in counter.values().copied() {
            if k <= c {
                rs += 1;
            }
        }
    }

    println!("{rs}");
}
