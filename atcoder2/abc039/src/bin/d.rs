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
        h: usize,
        w: usize,
        ss: [Chars; h],
    };
    let hh = h - 1;
    let ww = w - 1;
    let mut rs = vec![vec!['.'; w]; h];
    let mut check = vec![vec!['.'; w]; h];
    for i in 0..h {
        let mut xx = vec![i];
        if i != 0 {
            xx.push(i - 1);
        }
        if i != hh {
            xx.push(i + 1);
        }
        for j in 0..w {
            let mut yy = vec![j];
            if j != 0 {
                yy.push(j - 1);
            }
            if j != ww {
                yy.push(j + 1);
            }
            if iproduct!(&xx, &yy).all(|(&x, &y)| ss[x][y] == '#') {
                rs[i][j] = '#';
                for (&x, &y) in iproduct!(&xx, &yy) {
                    check[x][y] = '#';
                }
            }
        }
    }
    if ss == check {
        println!("possible");
        for rs in rs {
            println!("{}", rs.iter().join(""));
        }
    } else {
        println!("impossible");
    }
}
