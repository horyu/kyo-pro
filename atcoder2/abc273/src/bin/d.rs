#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
#![feature(let_chains)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut y: Usize1,
        mut x: Usize1,
        n: usize,
        rrcc: [(Usize1, Usize1); n],
        q: usize,
        ddll: [(char, usize); q],
    };
    let mut tates = HashMap::new();
    let mut yokos = HashMap::new();
    for (r, c) in rrcc {
        tates.entry(c).or_insert_with(BTreeSet::new).insert(r);
        yokos.entry(r).or_insert_with(BTreeSet::new).insert(c);
    }
    for (d, l) in ddll {
        match d {
            'L' => {
                if let Some(yoko) = yokos.get(&y) && let Some(&wall_x) = yoko.range(..x).next_back() {
                    x = (wall_x + 1).max(x.saturating_sub(l));
                } else {
                    x = x.saturating_sub(l);
                }
            }
            'R' => {
                if let Some(yoko) = yokos.get(&y) && let Some(&wall_x) = yoko.range(x..).next() {
                    x = (wall_x - 1).min(x + l);
                } else {
                    x = (x + l).min(w - 1);
                }
            }
            'U' => {
                if let Some(tate) = tates.get(&x) && let Some(&wall_y) = tate.range(..y).next_back() {
                    y = (wall_y + 1).max(y.saturating_sub(l));
                } else {
                    y = y.saturating_sub(l);
                }
            }
            _ => {
                if let Some(tate) = tates.get(&x) && let Some(&wall_y) = tate.range(y..).next() {
                    y = (wall_y - 1).min(y + l);
                } else {
                    y = (y + l).min(h - 1);
                }
            }
        }
        println!("{} {}", y + 1, x + 1);
    }
}
