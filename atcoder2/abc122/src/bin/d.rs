#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
#![feature(array_windows)]
use ac_library::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    ops::AddAssign,
};

fn main() {
    input! {
        n: usize,
    };
    // https://blog.hamayanhamayan.com/entry/2019/03/25/232610
    type HM = std::collections::HashMap<[u8; 3], ModInt1000000007>;
    let mut hm: HM = HashMap::new();
    // 01234: #AGCT
    hm.insert([0u8; 3], ModInt1000000007::new(1));
    fn is_ok(cc: &[u8], c: u8) -> bool {
        let mut arr = [cc[0], cc[1], cc[2], c];
        for i in 0..4 {
            if i != 0 {
                arr.swap(i - 1, i);
            }
            if matches!(arr, [1, 2, 3, _] | [_, 1, 2, 3]) {
                return false;
            }
            if i != 0 {
                arr.swap(i - 1, i);
            }
        }
        true
    }
    for i in 0..n {
        let mut new_hm: HM = HashMap::new();
        for (k, cnt) in hm {
            for new_char in 1..=4 {
                if is_ok(&k, new_char) {
                    new_hm
                        .entry([k[1], k[2], new_char])
                        .or_default()
                        .add_assign(cnt);
                }
            }
        }
        hm = new_hm;
    }
    let mut rs = ModInt1000000007::default();
    for (k, cnt) in hm {
        rs.add_assign(cnt);
    }
    println!("{rs}");
}
