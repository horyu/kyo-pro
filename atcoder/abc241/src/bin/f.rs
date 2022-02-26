#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        s: (usize, usize),
        g: (usize, usize),
        xxyy: [(usize, usize); n]
    };
    let mut xyhm = HashMap::new();
    let mut yxhm = HashMap::new();
    for &(x, y) in &xxyy {
        xyhm.entry(x).or_insert_with(BTreeSet::new).insert(y);
        yxhm.entry(y).or_insert_with(BTreeSet::new).insert(x);
    }
    let mut checked = HashMap::new();
    let mut qq = VecDeque::new();
    qq.push_back((s, 0));
    checked.insert(s, 0usize);
    while let Some(((qx, qy), c)) = qq.pop_front() {
        if let Some(ybtm) = xyhm.get(&qx) {
            if let Some(&l) = ybtm.range(0..qy).into_iter().rev().next() {
                let next_q = (qx, l + 1);
                if let hash_map::Entry::Vacant(e) = checked.entry(next_q) {
                    // eprintln!("l{}: ({},{})", c + 1, next_q.0, next_q.1);
                    e.insert(c + 1);
                    qq.push_back((next_q, c + 1));
                }
            }
            if let Some(&r) = ybtm.range((qy + 1)..std::usize::MAX).into_iter().next() {
                let next_q = (qx, r - 1);
                if let hash_map::Entry::Vacant(e) = checked.entry(next_q) {
                    // eprintln!("r{}: ({},{})", c + 1, next_q.0, next_q.1);
                    e.insert(c + 1);
                    qq.push_back((next_q, c + 1));
                }
            }
        }
        if let Some(xbtm) = yxhm.get(&qy) {
            if let Some(&u) = xbtm.range(0..qx).into_iter().rev().next() {
                let next_q = (u + 1, qy);
                if let hash_map::Entry::Vacant(e) = checked.entry(next_q) {
                    // eprintln!("u{}: ({},{})", c + 1, next_q.0, next_q.1);
                    e.insert(c + 1);
                    qq.push_back((next_q, c + 1));
                }
            }
            if let Some(&d) = xbtm.range((qx + 1)..std::usize::MAX).into_iter().next() {
                let next_q = (d - 1, qy);
                if let hash_map::Entry::Vacant(e) = checked.entry(next_q) {
                    // eprintln!("d{}: ({},{})", c + 1, next_q.0, next_q.1);
                    e.insert(c + 1);
                    qq.push_back((next_q, c + 1));
                }
            }
        }
    }

    if let Some(c) = checked.get(&g) {
        println!("{c}");
    } else {
        println!("-1");
    }
    // dbg!(checked);
    // for (x, btm) in xyhm {
    //     println!("x {x}:{}", btm.iter().join(" "));
    // }
    // for (y, btm) in yxhm {
    //     println!("y {y}:{}", btm.iter().join(" "));
    // }
}
