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
        k: isize,
        aa: [isize; n],
    };
    // https://atcoder.jp/contests/abc370/editorial/10858
    let mut hm = HashMap::new();
    hm.insert(0isize, ModInt998244353::new(1));
    let mut all = ModInt998244353::new(1);
    let mut acc = 0;
    for (i, a) in aa.into_iter().enumerate() {
        acc += a;
        let cur = all - hm.get(&(acc - k)).copied().unwrap_or_default();
        *hm.entry(acc).or_insert(ModInt998244353::default()) += cur;
        all += cur;
        if i + 1 == n {
            let rs = cur.val();
            println!("{rs}");
        }
    }
}
