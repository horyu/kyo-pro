#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        t: Chars,
        ss: [Chars; n],
    };
    let mut ii = vec![];
    for (i, s) in ss.iter().enumerate() {
        let diff = s.len() as isize - t.len() as isize;
        match diff {
            -1 => {
                // s.len() < t.len()
                let mut si = 0;
                let mut ti = 0;
                let mut cnt = 0;
                while si < s.len() && ti < t.len() {
                    if s[si] == t[ti] {
                        si += 1;
                        ti += 1;
                    } else {
                        ti += 1;
                        cnt += 1;
                    }
                }
                if cnt <= 1 {
                    ii.push(i);
                }
            }
            0 => {
                let mut cnt = 0;
                for (sc, tc) in izip!(s, &t) {
                    if sc != tc {
                        cnt += 1;
                    }
                }
                if cnt <= 1 {
                    ii.push(i);
                }
            }
            1 => {
                // s.len() > t.len()
                let mut si = 0;
                let mut ti = 0;
                let mut cnt = 0;
                while si < s.len() && ti < t.len() {
                    if s[si] == t[ti] {
                        si += 1;
                        ti += 1;
                    } else {
                        si += 1;
                        cnt += 1;
                    }
                }
                if cnt <= 1 {
                    ii.push(i);
                }
            }
            _ => (),
        }
    }
    println!("{}", ii.len());
    println!("{}", ii.into_iter().map(|i| i + 1).join(" "));
}
