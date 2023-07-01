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
        aa: [usize; n],
        s: Chars,
    };
    let mut mm = vec![vec![]; 3];
    let mut ee = vec![vec![]; 3];
    let mut xx = vec![vec![]; 3];
    for ((i, a), c) in izip!(aa.iter().copied().enumerate(), s) {
        match c {
            'M' => mm[a].push(i),
            'E' => ee[a].push(i),
            'X' => xx[a].push(i),
            _ => unreachable!(),
        }
    }
    let mut rs = 0;
    for (ea, ee) in ee.iter().enumerate() {
        for ei in ee.iter().copied() {
            let mmcc = mm
                .iter()
                .map(|mm| mm.partition_point(|&mi| mi <= ei))
                .collect_vec();
            let xxcc = xx
                .iter()
                .map(|xx| xx.len() - xx.partition_point(|&xi| xi <= ei))
                .collect_vec();
            // eprintln!("{ea} {ei:?}");
            // for (ma, mc) in mmcc.iter().enumerate() {
            //     eprintln!("ma:{ma} mc:{mc}");
            // }
            // for (xa, xc) in xxcc.iter().enumerate() {
            //     eprintln!("xa:{xa} xc:{xc}");
            // }

            for (ma, mc) in mmcc.iter().copied().enumerate() {
                for (xa, xc) in xxcc.iter().copied().enumerate() {
                    let mut ttff = [true; 4];
                    ttff[ea] = false;
                    ttff[ma] = false;
                    ttff[xa] = false;
                    rs += ttff.iter().position(|&tf| tf).unwrap() * mc * xc;
                }
            }
        }
    }
    println!("{rs}");
}
