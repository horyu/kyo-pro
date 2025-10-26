#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        r: usize,
        mut ll: [usize; n],
    };
    let Some(left) = ll.iter().position(|&l| l == 0) else {
        // 全てのドアが閉まっていたらOK
        println!("0");
        return;
    };
    let right = ll.iter().rposition(|&l| l == 0).unwrap();
    // ドアleft から ドアright の区間内にある全ドアを開ける
    let mut rs = 0;
    for i in left..right {
        if ll[i] == 1 {
            rs += 1;
            ll[i] = 0;
        }
    }
    // 高橋君の位置 r から空いている区間までドアを開ける
    if r <= left {
        for i in r..=left {
            if ll[i] == 1 {
                rs += 1;
                ll[i] = 0;
            }
        }
    } else if right <= r {
        for i in right..r {
            if ll[i] == 1 {
                rs += 1;
                ll[i] = 0;
            }
        }
    }
    // 全てのドアを閉める
    rs += ll.iter().filter(|&&l| l == 0).count();
    println!("{rs}");
}
