#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        a: u32,
        b: u32,
        c: usize,
    };
    eprintln!("{c:b}");
    let mut aa = a;
    let mut bb = b;
    let mut x = 0usize;
    let mut y = 0usize;
    for i in 0..60 {
        if 0 == c & 1 << i {
            continue;
        }
        // aa か bb の大きい方から引く
        if aa < bb {
            bb -= 1;
            y |= 1 << i;
        } else if 0 < aa {
            aa -= 1;
            x |= 1 << i;
        }
    }
    let mut amari = aa + bb;
    dbg!(amari);
    if amari % 2 == 1 {
        println!("-1");
        return;
    }
    for i in 0..60 {
        if amari == 0 {
            break;
        }
        if 0 == (c & 1 << i) {
            amari -= 2;
            x |= 1 << i;
            y |= 1 << i;
        }
    }
    eprintln!("{x:064b} {}", x.count_ones());
    eprintln!("{y:064b} {}", y.count_ones());
    eprintln!("{c:064b}");
    if x.count_ones() == a && y.count_ones() == b && x ^ y == c {
        println!("{x} {y}");
    } else {
        println!("-1");
    }
}
