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
        xxyy: [(isize, isize); n],
        m: usize,
    };
    let mut oopp = vec![];
    for _ in 0..m {
        input! {o: Usize1};
        if o <= 1 {
            oopp.push((o, 0));
        } else {
            input! {p: isize};
            oopp.push((o, p));
        }
    }
    input! {
        q: usize,
        aabb: [(usize, Usize1); q],
    };
    use ndarray::array;
    let mut mm = vec![array![[1, 0, 0], [0, 1, 0], [0, 0, 1]]];
    for (o, p) in oopp.iter().copied() {
        let last_m = mm.last().unwrap();
        let m = match o {
            0 => {
                array![[0, 1, 0], [-1, 0, 0], [0, 0, 1]]
            }
            1 => {
                array![[0, -1, 0], [1, 0, 0], [0, 0, 1]]
            }
            2 => {
                array![[-1, 0, p * 2], [0, 1, 0], [0, 0, 1]]
            }
            _ => {
                array![[1, 0, 0], [0, -1, p * 2], [0, 0, 1]]
            }
        };
        mm.push(m.dot(last_m));
    }
    for (a, b) in aabb {
        let (x, y) = xxyy[b];
        let v = array![x, y, 1];
        let rs = mm[a].dot(&v);
        println!("{} {}", rs[0], rs[1]);
    }
}
