#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::RemEuclidU32;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        x: isize,
        y: isize,
    };
    // 2 = |ax - by|
    let xabs = x.abs();
    let yabs = y.abs();

    if x == 0 {
        if yabs <= 2 {
            println!("{} 0", 2 / yabs);
        } else {
            println!("-1");
        }
        return;
    }
    if y == 0 {
        if xabs <= 2 {
            println!("0 {}", 2 / xabs);
        } else {
            println!("-1");
        }
        return;
    }

    // https://qiita.com/drken/items/b97ff231e43bce50199a#3-%E4%B8%80%E6%AC%A1%E4%B8%8D%E5%AE%9A%E6%96%B9%E7%A8%8B%E5%BC%8F-ax--by--c-%E3%81%AE%E6%95%B4%E6%95%B0%E8%A7%A3
    let gcd = x.gcd(&y);
    if 2 % gcd != 0 {
        println!("-1");
        return;
    }
    let (a, b) = extgcd(y, -x);
    let a = a * 2 / gcd;
    let b = b * 2 / gcd;
    println!("{a} {b}");
}

// 拡張ユークリッドの互除法
fn extgcd(a: isize, b: isize) -> (isize, isize) {
    if b == 0 {
        return (1, 0);
    }
    let (y, x) = extgcd(b, a % b);
    (x, y - a / b * x)
}
