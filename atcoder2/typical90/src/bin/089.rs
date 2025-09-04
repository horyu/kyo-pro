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
        k: isize,
        mut aa: [usize; n],
    };
    // https://github.com/E869120/kyopro_educational_90/blob/main/sol/089-05.cpp
    const MOD: isize = 1e9 as isize + 7;
    let a2i = aa
        .iter()
        .copied()
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<HashMap<_, _>>();
    let aa = chain!([0], aa.into_iter().map(|a| a2i[&a])).collect_vec();
    let mut cl = vec![0; n + 1];
    let mut l = n;
    let mut cnt = 0;
    let mut ft = ac_library::FenwickTree::new(a2i.len() + 2, 0isize);
    ft.add(aa[n], 1);
    for r in (1..=n).rev() {
        while 1 <= l && cnt <= k {
            l -= 1;
            cnt += ft.sum(..aa[l]);
            ft.add(aa[l], 1);
        }
        ft.add(aa[r], -1);
        cnt -= ft.sum((aa[r] + 1)..);
        cl[r] = l;
    }

    let mut dp = vec![0; n + 1];
    let mut ru = vec![0; n + 1];
    dp[0] = 1;
    ru[0] = 1;
    for i in 1..=n {
        if cl[i] == 0 {
            dp[i] = ru[i - 1];
        }
        if 1 <= cl[i] {
            dp[i] = (ru[i - 1] - ru[cl[i] - 1] + MOD) % MOD;
        }
        ru[i] = (ru[i - 1] + dp[i]) % MOD;
    }
    let rs = dp[n];
    println!("{rs}");
}
