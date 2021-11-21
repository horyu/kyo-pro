#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{cell::RefCell, rc::Rc};

fn main() {
    input! {
        q: usize,
        ttxx: [(usize, usize); q]
    };
    const N: usize = 1048576;
    let mut aa = vec![std::usize::MAX; N];
    use std::collections::BTreeSet;
    let mut bts: BTreeSet<_> = (0..N).collect();
    for (t, x) in ttxx {
        let xn = x % N;
        if t == 2 {
            if aa[xn] == std::usize::MAX {
                println!("-1");
            } else {
                println!("{}", aa[xn]);
            }
            continue;
        }
        if let Some(&pos) = bts.range(xn..).next() {
            bts.remove(&pos);
            aa[pos] = x;
        } else if let Some(&pos) = bts.iter().next() {
            bts.remove(&pos);
            aa[pos] = x;
        }
    }
}
