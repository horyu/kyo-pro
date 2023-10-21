#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_bigint::ToBigUint;
use num_integer::*;
use num_traits::ToPrimitive;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn main() {
    input! {
        n: usize,
        mut a: Usize1,
        k: String,
        bb: [Usize1; n],
    };
    let k = num_bigint::BigUint::from_str(&k).unwrap();
    let mut a2i = vec![!0; n];
    a2i[a] = 0;
    for r in 1usize.. {
        a = bb[a];
        if k == r.into() {
            let rs = a + 1;
            println!("{rs}");
            return;
        }
        if a2i[a] != !0 {
            let l = a2i[a];
            let loop_size = r - l;
            for _ in 0..((k - r.to_biguint().unwrap()) % loop_size)
                .to_u64()
                .unwrap()
            {
                a = bb[a];
            }
            let rs = a + 1;
            println!("{rs}");
            return;
        }
        a2i[a] = r;
    }
    let rs = a + 1;
    println!("{rs}");
}
