#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        n: usize,
        s: Chars,
    };
    let s = s.into_iter().map(|c| c == 'o').collect_vec();
    for ww in (0..2).map(|_| [false, true]).multi_cartesian_product() {
        let mut tmp = vec![false; n];
        tmp[0] = ww[0];
        tmp[1] = ww[1];
        for i in 1..(n - 1) {
            tmp[i + 1] = tmp[i - 1] ^ tmp[i] ^ s[i];
        }
        if tmp[n - 2] ^ tmp[n - 1] ^ tmp[0] == s[n - 1] && tmp[n - 1] ^ tmp[0] ^ tmp[1] == s[0] {
            println!(
                "{}",
                tmp.iter()
                    .copied()
                    .map(|b| if b { 'S' } else { 'W' })
                    .join("")
            );
            // for i in 0..n {
            //     eprintln!("{i} ({}){} {}-{}", tmp[i],s[i],  tmp[(n + i - 1) % n], tmp[(i + 1) % n]);
            // }
            return;
        }
    }

    println!("-1");
}
