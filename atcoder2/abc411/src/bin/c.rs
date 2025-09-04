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

fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [Usize1; q],
    };
    // btm[i] = len; 左端 i, 長さ len の連続した黒の区間
    let mut btm = BTreeMap::<usize, usize>::new();
    let mut ttff = vec![false; n];
    for a in aa {
        if ttff[a] {
            // Black -> White
            ttff[a] = false;
            assert!(!btm.contains_key(&(a + 1)));
            if let Some((&k, &v)) = btm.range(..=a).next_back() {
                // 区間の右端との比較
                match a.cmp(&(k + v - 1)) {
                    Ordering::Less => {
                        if a == k {
                            btm.remove(&a);
                            btm.insert(a + 1, v - 1);
                        } else {
                            // 分割
                            btm.insert(k, a - k);
                            btm.insert(a + 1, k + v - a - 1);
                        }
                    }
                    Ordering::Equal => {
                        if v == 1 {
                            btm.remove(&k);
                        } else {
                            btm.insert(k, v - 1);
                        }
                    }
                    Ordering::Greater => {
                        unreachable!();
                    }
                }
            } else {
                unreachable!();
            }
        } else {
            // White -> Black
            ttff[a] = true;
            assert!(!btm.contains_key(&a));
            let len = if let Some(v) = btm.remove(&(a + 1)) {
                v + 1
            } else {
                1
            };
            if let Some((&k, &v)) = btm.range(..=a).next_back() {
                // 区間の右端+1との比較
                match a.cmp(&(k + v)) {
                    Ordering::Less => {
                        unreachable!();
                    }
                    Ordering::Equal => {
                        btm.insert(k, v + len);
                    }
                    Ordering::Greater => {
                        btm.insert(a, len);
                    }
                }
            } else {
                btm.insert(a, len);
            }
        }
        // eprintln!("{}", ttff.iter().map(|&x| if x { '@' } else { '_' }).join(""));
        // eprintln!("{btm:?}");
        let rs = btm.len();
        println!("{rs}");
    }
}
