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
        aa: [usize; n],
    };
    let mut keta_cntss = [0; 12];
    let mut keta_sums = [ModInt998244353::default(); 12];
    let mut rs = ModInt998244353::default();
    for (i, a) in aa.iter().copied().enumerate() {
        let keta = 1 + a.ilog10();
        for (&kc, &ks) in izip!(&keta_cntss, &keta_sums) {
            rs += ks * 10usize.pow(keta) + kc * a;
        }
        keta_cntss[keta as usize] += 1;
        keta_sums[keta as usize] += a;
    }
    println!("{rs}");
}
