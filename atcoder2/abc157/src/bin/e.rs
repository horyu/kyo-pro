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
        s: Bytes,
        q: usize,
    };
    let mut s = s.into_iter().map(|b| (b - b'a') as usize).collect_vec();
    let mut bbttss = vec![BTreeSet::<usize>::new(); 26];
    for (i, &c) in s.iter().enumerate() {
        bbttss[c].insert(i);
    }
    for _ in 0..q {
        input! {x: usize};
        if x == 1 {
            input! {i: Usize1, c: char};
            let c = (c as u8 - b'a') as usize;
            let ori_c = s[i];
            if c != ori_c {
                bbttss[ori_c].remove(&i);
                bbttss[c].insert(i);
                s[i] = c;
            }
        } else {
            input! {l: Usize1, r: Usize1};
            let mut rs = 0;
            for bts in &bbttss {
                if bts.range(l..=r).next().is_some() {
                    rs += 1;
                }
            }
            println!("{rs}");
        }
    }
}
