#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: Chars
    };
    let mut hm = HashMap::new();
    hm.insert([0; 2], ModInt998244353::new(1));
    for (i, c) in s.iter().copied().enumerate() {
        let mut new_hm = HashMap::new();
        for (mut kk, cnt) in hm {
            match c {
                '(' => {
                    if 0 < kk[1] {
                        kk[1] -= 1;
                    } else {
                        kk[0] += 1;
                    }
                    *new_hm.entry(kk).or_default() += cnt;
                }
                ')' => {
                    if 0 < kk[0] {
                        kk[0] -= 1;
                        *new_hm.entry(kk).or_default() += cnt;
                    }
                }
                _ => {
                    // )
                    let mut tmp = kk;
                    if 0 < tmp[0] {
                        tmp[0] -= 1;
                        *new_hm.entry(tmp).or_default() += cnt;
                    }
                    // (
                    let mut tmp = kk;
                    if 0 < tmp[1] {
                        tmp[1] -= 1;
                    } else {
                        tmp[0] += 1;
                    }
                    *new_hm.entry(tmp).or_default() += cnt;
                }
            }
        }
        hm = new_hm;
        // eprintln!("{i} {c} {}", hm.len());
    }
    let rs = hm.get(&[0; 2]).copied().unwrap_or_default();
    println!("{rs}");
}
