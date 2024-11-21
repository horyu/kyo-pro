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
        s: Chars,
        q: usize,
    };
    let mut bbttss = vec![BTreeSet::default(); 128];
    for (i, c) in s.iter().copied().enumerate() {
        bbttss[c as usize].insert(i + 1);
    }
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {i: usize, c: char};
            for bts in bbttss.iter_mut() {
                bts.remove(&i);
            }
            bbttss[c as usize].insert(i);
            continue;
        }
        input! {l: usize, r: usize};
        let mut rs = 0;
        for bts in bbttss.iter() {
            if bts.range(l..=r).next().is_some() {
                rs += 1;
            }
        }
        println!("{rs}");
    }
}
