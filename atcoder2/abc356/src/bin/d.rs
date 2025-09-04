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
        n: usize,
        m: usize,
    };
    // for n in 0usize..100 { for m in 0usize..100 {
    // let (n, m) = (2usize, 1usize);{{
    let mut rs = ModInt998244353::default();
    for d in 0u64..60 {
        if m & (1 << d) == 0 {
            continue;
        }
        // 1以上n以下のdビット目が立っている数の個数
        rs += n / (2 << d) * (1 << d);
        rs += (n % (2 << d)).saturating_sub((1 << d) - 1);
    }
    println!("{rs}");
    // if n < 100 && m < 100 {
    //     let mut tmp = 0u32;
    //     for k in 0..=n {
    //         tmp += (k & m).count_ones();
    //     }
    //     assert_eq!(rs.val(), tmp, "n={n} m={m}");
    // }
    // }}
}
