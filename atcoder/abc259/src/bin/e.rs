#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    // p => { e => [index] }
    let mut outer_btm = BTreeMap::new();
    let mut pppeee = vec![];
    for i in 0..n {
        input! {
            m: usize,
            ppee: [(usize, usize); m],
        }
        for &(p, e) in &ppee {
            outer_btm
                .entry(p)
                .or_insert_with(BTreeMap::new)
                .entry(e)
                .or_insert_with(HashSet::new)
                .insert(i);
        }
        pppeee.push(ppee);
    }
    let mut hs = HashSet::new();
    for i in 0..n {
        // p と e が単独で最小公倍数に貢献していたのなら
        // 1に置換することで最小公倍数が変化する
        let mut vv = vec![];
        for &(p, e) in &pppeee[i] {
            if let Some(inner_btm) = outer_btm.get(&p) {
                let mut range = inner_btm.range(e..);
                // 次要素がなければ最大の要素
                if let (Some(ehs), None) = (range.next(), range.next()) {
                    // 単独ならば
                    if ehs.1.len() == 1 {
                        vv.push((p, e));
                    }
                }
            }
        }
        hs.insert(vv);
    }
    println!("{}", hs.len());
}
