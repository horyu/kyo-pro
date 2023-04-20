#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        t: usize,
        nnsskk: [(usize, usize, usize); t],
    };
    for (n, s, k) in nnsskk {
        // kx = b mod n
        let d = k.gcd(&(n - s)).gcd(&n);
        let k = k / d;
        let b = (n - s) / d;
        let n = n / d;
        if k.gcd(&n) != 1 {
            println!("-1");
            continue;
        }
        ModInt::set_modulus(n as u32);
        let rs = ModInt::new(b) / k;
        println!("{rs}");
    }
}
