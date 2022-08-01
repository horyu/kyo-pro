#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
        mut bb: [usize; n],
    };
    // ABCA
    // AABC
    // BAAC
    // BCAA
    // BACA
    // ACBA
    let a_counts = aa.iter().counts();
    // 同じ個数か
    let mut tf = a_counts == bb.iter().counts();
    if tf {
        // 重複がないなら愚直にやって検証
        if a_counts.values().all(|&cnt| cnt == 1) {
            for l in 0..(n - 3) {
                let b = bb[l];
                if aa[l] != b {
                    let mut j = aa.iter().position(|&a| a == b).unwrap();
                    while l + 2 <= j {
                        aa.as_mut_slice()[(j - 2)..=j].rotate_right(1);
                        j -= 2;
                    }
                    if l < j {
                        aa.as_mut_slice()[l..=(l + 2)].rotate_right(2);
                    }
                }
            }
            tf = (0..3).any(|_| {
                aa.as_mut_slice()[(n - 3)..].rotate_right(1);
                ((n - 3)..n).all(|i| aa[i] == bb[i])
            });
        }
    }
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
