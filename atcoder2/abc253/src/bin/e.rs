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
        m: usize,
        k: usize,
    };
    if k == 0 {
        // 2 3 0
        // 1-[1,2,3], 2-[1,2,3], 3-[1,2,3]
        let rs = ModInt998244353::new(m).pow(n as u64);
        println!("{rs}");
        return;
    }

    let mut dp = vec![ModInt998244353::new(1); m];
    for _ in 1..n {
        let mut imos = vec![ModInt998244353::default(); m];
        for (i, v) in dp.iter().enumerate() {
            if let Some(l) = i.checked_sub(k) {
                imos[0] += v;
                imos[l + 1] -= v;
            }
            if i + k < m {
                imos[i + k] += v
            }
        }
        for i in 1..m {
            let tmp = imos[i - 1];
            imos[i] += tmp;
        }
        dp = imos;
    }
    let rs = dp.into_iter().sum::<ModInt998244353>();
    println!("{rs}");
}
