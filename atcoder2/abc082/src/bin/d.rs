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
        s: Chars,
        x: isize,
        y: isize,
    };
    let first_f = s[0] == 'F';
    let mut aa = vec![];
    let mut bb = vec![];
    let mut muki = false;
    for (k, g) in s.into_iter().group_by(|c| *c).into_iter() {
        if k == 'F' {
            [&mut aa, &mut bb][muki as usize].push(g.count() as isize);
        } else {
            if g.count().is_odd() {
                muki = !muki;
            }
        }
    }
    let xtf = if aa.is_empty() {
        x == 0
    } else {
        let mut hs = HashSet::new();
        if first_f {
            hs.insert(aa[0]);
            aa.swap_remove(0);
        } else {
            hs.insert(0);
        }
        for a in aa.into_iter() {
            let mut new_hs = HashSet::new();
            for p in hs {
                new_hs.insert(p + a);
                new_hs.insert(p - a);
            }
            hs = new_hs;
        }
        hs.contains(&x)
    };
    let ytf = if bb.is_empty() {
        y == 0
    } else {
        let mut hs = HashSet::new();
        hs.insert(0);
        for b in bb.into_iter() {
            let mut new_hs = HashSet::new();
            for q in hs {
                new_hs.insert(q + b);
                new_hs.insert(q - b);
            }
            hs = new_hs;
        }
        hs.contains(&y)
    };
    let tf = xtf && ytf;
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
