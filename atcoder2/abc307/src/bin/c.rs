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
        ha: usize,
        wa: usize,
        aa: [Chars; ha],
        hb: usize,
        wb: usize,
        bb: [Chars; hb],
        hx: usize,
        wx: usize,
        xx: [Chars; hx],
    };
    fn to_hs(vvv: &[Vec<char>]) -> HashSet<(isize, isize)> {
        let mut rs = vec![];
        for (i, vv) in vvv.iter().enumerate() {
            for (j, &v) in vv.iter().enumerate() {
                if v == '#' {
                    rs.push((i as isize, j as isize));
                }
            }
        }
        let imin = rs.iter().copied().map(|ij| ij.0).min().unwrap();
        let jmin = rs.iter().copied().map(|ij| ij.1).min().unwrap();
        rs.into_iter().map(|(i, j)| (i - imin, j - jmin)).collect()
    }
    let hsa = to_hs(&aa);
    let hsb = to_hs(&bb);
    let vvx = to_hs(&xx).into_iter().sorted_unstable().collect_vec();

    let base: HashSet<(isize, isize)> =
        hsa.iter().copied().map(|(i, j)| (i + 10, j + 10)).collect();
    for x in 0..30 {
        for y in 0..30 {
            let mut hs = base.clone();
            for (i, j) in hsb.iter().copied() {
                hs.insert((i + x, j + y));
            }
            if hs.len() == vvx.len()
                && izip!(hs.iter().copied().sorted_unstable(), vvx.iter().copied())
                    .map(|(p, q)| (p.0 - q.0, p.1 - q.1))
                    .all_equal()
            {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
