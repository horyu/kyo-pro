#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        t: usize,
        nn: [usize; t],
    };
    for n in nn {
        let sqrt = n.sqrt();
        let mut rs = ModInt998244353::default();
        // 1<=k<=sqrt の範囲でx,y,zのうち少なくとも1つはkとなる組み合わせ
        // for k in 1..=sqrt {
        //     rs += ModInt998244353::new(k).pow(3) - ModInt998244353::new(k - 1).pow(3);
        // }
        rs += ModInt998244353::new(sqrt).pow(3);
        // sqrt<k<=n の範囲で xy,yz,zx <= n となる組み合わせ
        // 25 => 5*5
        // 6*4, 7~8*3, 9~12*2, 13~25*1
        // 1*(25~13), 2*(12~9), 3*(8~7), 4*(6~6)
        if n < 100 {
            eprintln!("[{n}]({sqrt}): {rs}");
        }
        for i in 1..sqrt {
            let r = n / i;
            let l = n / (i + 1) + 1;
            // 2つの値が i, 1つの値が l..=r となる組み合わせ
            rs += 3 * (r - l + 1);
            if n < 100 {
                eprintln!("{i} +{:?}*3={}*3 => {rs}", (l..=r), r - l + 1);
            }
        }
        println!("{rs}");
    }
}
