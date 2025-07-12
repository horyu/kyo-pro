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
        mut n: usize,
        q: usize,
        xxyy: [(isize, isize); n],
    };
    let mut dsu = ac_library::Dsu::new(n + q);
    let mut xxyy = xxyy.into_iter().map(|(x, y)| (x + y, x - y)).collect_vec();
    let mut mm = btreemultimap::BTreeMultiMap::new();
    let d = |a: (isize, isize), b: (isize, isize)| (a.0.abs_diff(b.0)).max(a.1.abs_diff(b.1));
    for (i, ixy) in xxyy.iter().copied().enumerate() {
        for (j, jxy) in xxyy.iter().copied().enumerate().skip(i + 1) {
            mm.insert(d(ixy, jxy), (i, j));
        }
    }
    for _ in 0..q {
        input! {t: usize};
        match t {
            1 => {
                input! {a: isize, b: isize};
                let xy = (a + b, a - b);
                let mut tmp = multimap::MultiMap::new();
                for (i, ixy) in xxyy.iter().copied().enumerate() {
                    let l = dsu.leader(i);
                    tmp.insert(l, d(xy, ixy));
                }
                for (l, dd) in tmp {
                    let min = dd.into_iter().min().unwrap();
                    mm.insert(min, (l, n));
                }
                xxyy.push(xy);
                n += 1;
            }
            2 => {
                let mut merged = false;
                while let Some((k, iijj)) = mm
                    .keys()
                    .next()
                    .copied()
                    .map(|k| (k, mm.remove(&k).unwrap()))
                {
                    for (i, j) in iijj {
                        if dsu.same(i, j) {
                            continue;
                        }
                        dsu.merge(i, j);
                        merged = true;
                    }
                    if merged {
                        println!("{k}");
                        break;
                    }
                }
                if !merged {
                    println!("-1");
                }
            }
            _ => {
                input! {u: Usize1, v: Usize1};
                let tf = dsu.same(u, v);
                let rs = ["No", "Yes"][tf as usize];
                println!("{rs}");
            }
        }
    }
}
