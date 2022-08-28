#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_bigint::ToBigUint;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn main() {
    input! {
        n: usize,
        a: Usize1,
        k: String,
        bb: [Usize1; n],
    };
    let k = num_bigint::BigUint::from_str(&k).unwrap();

    let mut b_to_i = vec![std::usize::MAX; n];
    let mut b = a;
    b_to_i[b] = 0;
    for i in 1..(2 * n) {
        let new_b = bb[b];
        // eprintln!("{i} {b}->{new_b}");
        if k == i.to_biguint().unwrap() {
            println!("{}", new_b + 1);
            return;
        }
        if b_to_i[new_b] != std::usize::MAX {
            let pre_i = b_to_i[new_b];
            let loop_size = i - pre_i;
            // dbg!(i, &k, pre_i, loop_size, (&k - pre_i) % loop_size);
            let from_loop_start = *((k - pre_i) % loop_size)
                .to_u64_digits()
                .first()
                .unwrap_or(&0) as usize;
            b = new_b;
            for _ in 0..from_loop_start {
                b = bb[b];
            }
            println!("{}", b + 1);
            return;
        }
        b_to_i[new_b] = i;
        b = new_b;
    }
}
