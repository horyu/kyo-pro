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
        m: usize,
    };
    let a = h / 10;
    let b = h % 10;
    for mm in m..60 {
        let c = mm / 10;
        let d = mm % 10;
        // eprintln!("{a}{b} {c}{d}");
        if (0..=5).contains(&b)
            && (100 * h + mm) <= (1000 * a + 100 * b + 10 * c + d)
            && 10 * a + c < 24
        {
            println!("{a}{b} {c}{d}");
            return;
        }
    }
    for hh in (h + 1)..24 {
        let a = hh / 10;
        let b = hh % 10;
        for mm in 0..60 {
            let c = mm / 10;
            let d = mm % 10;
            // eprintln!("{a}{b} {c}{d}");
            if (0..=5).contains(&b) && (100 * hh + mm) <= (1000 * a + 100 * b + 10 * c + d) {
                println!("{a}{b} {c}{d}");
                return;
            }
        }
    }
    for hh in 0..h {
        let a = hh / 10;
        let b = hh % 10;
        for mm in 0..60 {
            let c = mm / 10;
            let d = mm % 10;
            // eprintln!("{a}{b} {c}{d}");
            if (0..=5).contains(&b) && (100 * hh + mm) <= (1000 * a + 100 * b + 10 * c + d) {
                println!("{a}{b} {c}{d}");
                return;
            }
        }
    }
    // println!("{rs}");
}
