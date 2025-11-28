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
        q: usize,
    };
    let mut r2l = BTreeMap::new();
    let mut btm = BTreeMap::new();
    let mut s = vec![];
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {c: char};
            let slen = s.len();
            s.push(c);
            if c == ')' {
                if let Some((&pos, &cc)) = btm.last_key_value()
                    && cc == '('
                {
                    btm.pop_last();
                    r2l.insert(slen, pos);
                } else {
                    btm.insert(slen, c);
                }
            } else {
                btm.insert(slen, c);
            }
        } else {
            btm.pop_last();
            let c = s.pop().unwrap();
            if c == ')' {
                let slen = s.len();
                if let Some(pos) = r2l.remove(&slen) {
                    btm.insert(pos, '(');
                }
            }
        }
        eprintln!("{s:?} {btm:?} {r2l:?}");
        let rs = ["No", "Yes"][btm.is_empty() as usize];
        println!("{rs}");
    }
}
