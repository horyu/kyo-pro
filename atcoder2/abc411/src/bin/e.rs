#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

/*
3面ダイス3つで考える
a:1  45
b: 2 4  7
c:  3  67
P(X) = ダイスの最大値がXとなる組み合わせ
P(1) = 0; P(2) = 0; P(3) = 1;
P(4) = 1+2 = 3; P(5) = 2;
P(6) = 3*2 = 6; P(7) = 3*2+3*3 = 15;
*/

fn main() {
    input! {
        n: usize,
        aaa: [[usize; 6]; n],
    };
    let aaii = aaa
        .into_iter()
        .enumerate()
        .flat_map(|(i, aa)| aa.into_iter().map(move |a| (a, i)))
        .sorted_unstable();
    let mut i2c = vec![0; n];
    let mut c_sum = [0u64; 7];
    c_sum[0] += n as u64;
    let div = ModInt998244353::new(1) / ModInt998244353::new(6).pow(n as u64);
    let mm = (0..=6).map(ModInt998244353::new).collect_vec();
    let mut rs = ModInt998244353::default();
    for (a, i) in aaii {
        c_sum[i2c[i]] -= 1;
        if c_sum[0] == 0 {
            let mut tmp = ModInt998244353::new(1);
            for (m, &s) in izip!(&mm, &c_sum).skip(1) {
                if s != 0 {
                    tmp *= m.pow(s);
                }
            }
            rs += tmp * a * div;
        }
        i2c[i] += 1;
        c_sum[i2c[i]] += 1;
    }
    println!("{rs}");
}
