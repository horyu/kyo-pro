#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        n: usize,
        aa: [isize; n],
    };
    print!("{n}");
    for k in 2..=n {
        // aa[i] が先頭の等差数列になる長さk部分列の数
        let mut rs = ModInt998244353::default();
        for (i, ia) in aa.iter().copied().enumerate() {
            // hm[(等差d, 長さl)] = 数
            let mut hm = HashMap::new();
            for ja in aa.iter().copied().skip(i + 1) {
                let mut new_hm = hm.clone();
                *new_hm
                    .entry((ja - ia, 2))
                    .or_insert_with(ModInt998244353::default) += 1;
                for ((d, l), c) in hm {
                    if ja == ia + d * l && l < k as isize {
                        *new_hm
                            .entry((d, l + 1))
                            .or_insert_with(ModInt998244353::default) += c;
                    }
                }
                hm = new_hm;
            }
            for ((_, l), c) in hm {
                if l == k as isize {
                    rs += c;
                }
            }
        }
        print!(" {rs}");
    }
    println!();
}
