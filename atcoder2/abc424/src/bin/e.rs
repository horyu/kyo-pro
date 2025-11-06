#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct F64Ord(pub f64);
impl Eq for F64Ord {}
impl PartialOrd for F64Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for F64Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            mut k: usize,
            mut x: usize,
            aa: [usize; n],
        };
        let mut btm = BTreeMap::new();
        for (cnt, a) in aa.into_iter().sorted_unstable().dedup_with_count() {
            btm.insert(F64Ord(a as f64), cnt);
        }
        while 0 < k {
            let (F64Ord(a), c) = btm.pop_last().unwrap();
            match k.cmp(&c) {
                Ordering::Less => {
                    btm.insert(F64Ord(a), c - k);
                    let cnt = k * 2;
                    btm.entry(F64Ord(a / 2.0))
                        .and_modify(|v| *v += cnt)
                        .or_insert(cnt);
                    break;
                }
                Ordering::Equal => {
                    let cnt = c * 2;
                    btm.entry(F64Ord(a / 2.0))
                        .and_modify(|v| *v += cnt)
                        .or_insert(cnt);
                    break;
                }
                Ordering::Greater => {
                    k -= c;
                    let cnt = c * 2;
                    btm.entry(F64Ord(a / 2.0))
                        .and_modify(|v| *v += cnt)
                        .or_insert(cnt);
                }
            }
        }
        for (F64Ord(a), c) in btm.into_iter().rev() {
            if x <= c {
                println!("{a:.20}");
                break;
            }
            x -= c;
        }
    }
}
