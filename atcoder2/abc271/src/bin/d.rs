#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s: usize,
        aabb: [(usize, usize); n],
    };
    let mut hm = HashMap::new();
    hm.insert(0, String::new());
    for (a, b) in aabb {
        let mut new_hm = HashMap::new();
        for (k, mut ss) in hm {
            if k + a <= s {
                let mut new_ss = ss.clone();
                new_ss.push('H');
                new_hm.insert(k + a, new_ss);
            }
            if a != b && k + b <= s {
                ss.push('T');
                new_hm.insert(k + b, ss);
            }
        }
        hm = new_hm;
        // dbg!(&hm);
    }
    if let Some(ss) = hm.get(&s) {
        println!("Yes\n{ss}");
    } else {
        println!("No");
    }
}
