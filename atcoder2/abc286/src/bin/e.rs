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
        ss: [Chars; n],
        q: usize,
        uuvv: [(Usize1, Usize1); q],
    };
    let mut ddcc = vec![vec![(!0usize, 0); n]; n];
    for i in 0..n {
        ddcc[i][i].0 = 0;
    }
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.iter().enumerate() {
            if *c == 'Y' {
                ddcc[i][j] = (1, aa[i] + aa[j]);
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let (od, oc) = ddcc[i][j];
                let (nd, nc) = (
                    ddcc[i][k].0.saturating_add(ddcc[k][j].0),
                    (ddcc[i][k].1 + ddcc[k][j].1).saturating_sub(aa[k]),
                );
                if nd < od || (nd == od && oc < nc) {
                    ddcc[i][j] = (nd, nc);
                }
            }
        }
    }
    for (u, v) in uuvv {
        let (d, c) = ddcc[u][v];
        if d == !0 {
            println!("Impossible");
        } else {
            println!("{d} {c}");
        }
    }
    // for i in 0..n {
    //     eprintln!("{}", dd[i].iter().map(|dc| dc.0).join(" "));
    // }
    // eprintln!();
    // for i in 0..n {
    //     eprintln!("{}", dd[i].iter().map(|dc| dc.1).join(" "));
    // }
}
