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
        n: usize,
        qq: [(usize, usize, usize, usize); n],
    };
    const SIZE: usize = 1001;
    let mut imos = vec![vec![0isize; SIZE]; SIZE];
    for (lx, ly, rx, ry) in qq {
        imos[lx][ly] += 1;
        imos[lx][ry] -= 1;
        imos[rx][ly] -= 1;
        imos[rx][ry] += 1;
    }
    for i in 0..SIZE {
        for j in 1..SIZE {
            imos[i][j] += imos[i][j - 1];
        }
    }
    for i in 1..SIZE {
        for j in 0..SIZE {
            imos[i][j] += imos[i - 1][j];
        }
    }
    let mut rrss = vec![0; n];
    for imo in imos {
        for im in imo {
            if 0 < im {
                rrss[im as usize - 1] += 1;
            }
        }
    }
    let rs = rrss.iter().join("\n");
    println!("{rs}");
}
