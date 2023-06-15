#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        l: u128,
        r: u128,
    };
    let llog = l.ilog10();
    let rlog = r.ilog10();
    let mut rs = ModInt1000000007::new(0);
    if llog == rlog {
        let n = r - l + 1;
        // dbg!(n);
        rs += ModInt1000000007::new(l + r) * n / 2 * (llog + 1);
    } else {
        // l:92(1) 92~99=8=100-91-1
        let n = 10u128.pow(llog + 1) - l;
        // dbg!(n);
        rs += ModInt1000000007::new(2 * l + n - 1) * n / 2 * (llog + 1);
        // dbg!(rs);
        // r:1016(3) 1000~1016=17=r-1000+1
        let n = r - 10u128.pow(rlog) + 1;
        rs += ModInt1000000007::new(2 * 10u128.pow(rlog) + n - 1) * n / 2 * (rlog + 1);
        // dbg!(rs);
        for log in (llog + 1)..rlog {
            // 100~999=900=1000-100
            let n = 10u128.pow(log + 1) - 10u128.pow(log);
            // dbg!(n);
            rs += ModInt1000000007::new(2 * 10u128.pow(log) + n - 1) * n / 2 * (log + 1);
            // dbg!(rs);
        }
    }
    println!("{rs}");
}
