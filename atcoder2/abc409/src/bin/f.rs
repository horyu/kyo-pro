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
    let mut bh = BinaryHeap::new();
    let mut mm = multimap::MultiMap::new();
    let d = |a: (isize, isize), b: (isize, isize)| (a.0.abs_diff(b.0)).max(a.1.abs_diff(b.1));
    for (i, ixy) in xxyy.iter().copied().enumerate() {
        for (j, jxy) in xxyy.iter().copied().enumerate().skip(i + 1) {
            let d = d(ixy, jxy);
            if !mm.contains_key(&d) {
                bh.push(R(d));
            }
            mm.insert(d, (i, j));
        }
    }
    let mut rs = String::new();
    for _ in 0..q {
        input! {t: usize};
        match t {
            1 => {
                input! {a: isize, b: isize};
                let xy = (a + b, a - b);
                let mut hm = HashMap::<usize, usize>::new();
                for (i, ixy) in xxyy.iter().copied().enumerate() {
                    let l = dsu.leader(i);
                    let d = d(xy, ixy);
                    hm.entry(l).and_modify(|e| *e = (*e).min(d)).or_insert(d);
                }
                for (l, d) in hm {
                    if !mm.contains_key(&d) {
                        bh.push(R(d));
                    }
                    mm.insert(d, (l, n));
                }
                xxyy.push(xy);
                n += 1;
            }
            2 => {
                let mut merged = false;
                while let Some((k, iijj)) = bh.pop().map(|R(k)| (k, mm.remove(&k).unwrap())) {
                    for (i, j) in iijj {
                        if dsu.same(i, j) {
                            continue;
                        }
                        dsu.merge(i, j);
                        merged = true;
                    }
                    if merged {
                        rs.push_str(k.to_string().as_str());
                        rs.push('\n');
                        break;
                    }
                }
                if !merged {
                    rs.push_str("-1\n");
                }
            }
            _ => {
                input! {u: Usize1, v: Usize1};
                let tf = dsu.same(u, v);
                let noyes = ["No\n", "Yes\n"][tf as usize];
                rs.push_str(noyes);
            }
        }
    }
    print!("{rs}");
}
