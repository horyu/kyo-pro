#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: isize,
        ddd: [(isize, isize); 3],
        // a: isize,
        // b: isize,
        // c: isize,
        // d: isize,
        // e: isize,
        // f: isize,
        xxyy: [(isize, isize); m],
    };
    let hs: HashSet<(isize, isize)> = xxyy.into_iter().collect();
    let mut hm = HashMap::new();
    hm.insert((0isize, 0isize), ModInt998244353::new(1));
    for _ in 0..n {
        let mut new_hm = HashMap::new();
        for (&(qx, qy), v) in &hm {
            for &(dx, dy) in &ddd {
                let xy = (qx + dx, qy + dy);
                if hs.contains(&xy) {
                    continue;
                }
                if let Some(e) = new_hm.get_mut(&xy) {
                    *e += v;
                } else {
                    new_hm.insert(xy, *v);
                }
            }
        }
        hm = new_hm;
    }
    let rs = hm.into_values().sum::<ModInt998244353>();
    println!("{rs}");
}
