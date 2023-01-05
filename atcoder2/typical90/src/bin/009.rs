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
        xxyy: [(f64, f64); n],
    };
    let mut rs = 0.0f64;
    for i in 0..n {
        let (ox, oy) = xxyy[i];
        let qq = chain!(0..i, (i + 1)..n)
            .map(|j| {
                let (x, y) = xxyy[j];
                let xx = x - ox;
                let yy = y - oy;
                // (1, 0)-(0, 0)-(x - ox, y - oy) のなす角度
                let cos = (x - ox) / (xx.powi(2) + yy.powi(2)).sqrt();
                let mut theta = cos.acos() * std::f64::consts::FRAC_1_PI * 180.0;
                if yy.is_sign_negative() {
                    theta = 360.0 - theta;
                }
                theta
            })
            .sorted_unstable_by(|a, b| a.partial_cmp(b).unwrap())
            .collect_vec();
        for j in 0..(n - 1) {
            let qj = qq[j];
            let k = qq.partition_point(|&q| q <= qj + 180.0);
            for kk in [k.saturating_sub(1), k, k + 1] {
                if let Some(qk) = qq.get(kk) {
                    let abs = (qj - qk).abs();
                    let tmp = abs.min(360.0 - abs);
                    rs = rs.max(tmp);
                }
            }
        }
    }
    println!("{rs}");
}
