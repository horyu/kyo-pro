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
        n: usize,
        mut m: usize,
        ttxx: [(usize, usize); n],
    };
    let mut aa = vec![];
    let mut bb = vec![];
    let mut cc = vec![];
    for (t, x) in ttxx {
        match t {
            0 => {
                aa.push(x);
            }
            1 => {
                bb.push(x);
            }
            _ => {
                cc.push(x);
            }
        }
    }
    if cc.is_empty() {
        println!(
            "{}",
            aa.into_iter()
                .sorted_unstable()
                .rev()
                .take(m)
                .sum::<usize>()
        );
        return;
    }

    bb.sort_unstable();
    cc.sort_unstable();
    while aa.len() < m && !bb.is_empty() && !cc.is_empty() {
        m -= 1;
        for _ in 0..cc.pop().unwrap() {
            if let Some(b) = bb.pop() {
                aa.push(b);
            } else {
                break;
            }
        }
    }
    aa.sort_unstable();
    aa.reverse();
    aa.truncate(m);
    let mut sum = aa.iter().copied().sum::<usize>();
    let mut rs = sum;
    let mut bts = BTreeSet::new();
    for (ai, a) in aa.iter().copied().enumerate() {
        bts.insert((a, ai, 0));
    }
    bb.reverse();
    cc.reverse();
    let mut bi = 0;
    for c in cc {
        // c を開けるため1回消費
        if let Some((p, pi, pt)) = bts.pop_first() {
            // eprintln!("{sum} -> {}", sum - p);
            sum -= p;
        } else {
            break;
        }
        for _ in 0..c {
            if let Some(b) = bb.get(bi).copied() {
                if let Some((a, ai, 0)) = bts.first().copied() {
                    if a < b {
                        // eprintln!("swap {a} {b}");
                        bts.pop_first();
                        bts.insert((b, bi, 1));
                        sum += b - a;
                    } else {
                        bi = n;
                    }
                }
            } else {
                break;
            }
            bi += 1;
        }
        rs = rs.max(sum);
    }
    println!("{rs}");
}
