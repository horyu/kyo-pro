#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        l: u128,
        r: u128,
    };
    // 1からnまでのiに対して、iをi回繰り返した文字列長の総和を求める
    fn f(n: u128) -> ModInt1000000007 {
        if n == 0 {
            return Default::default();
        }
        let mut sum = ModInt1000000007::default();
        for digit in 1..=(n.ilog10() + 1) {
            let upper = if digit == n.ilog10() + 1 {
                n
            } else {
                10u128.pow(digit) - 1
            };
            let lower = 10u128.pow(digit - 1);
            sum += digit as u128 * (upper + lower) * (upper - lower + 1) / 2;
            // eprintln!("{digit} {upper} {lower} {sum}");
        }
        sum
    }
    let rs = f(r) - f(l - 1);
    println!("{rs}");
}
