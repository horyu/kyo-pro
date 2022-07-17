#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;
 
fn main() {
    input! {
        n: usize,
        k: usize,
        pp: [Usize1; n],
    };
    let mut rrss = vec![0; n];
    if k == 1 {
        for (i, p) in pp.into_iter().enumerate() {
            rrss[p] = i + 1;
        }
    } else {
        let mut btm: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (i, p) in pp.into_iter().enumerate() {
            if let Some((&q, _)) = btm.range(p..).next() {
                let mut vv = btm.remove(&q).unwrap();
                if vv.len() + 1 == k {
                    // eprintln!("remove {p},{}", vv.iter().join(","));
                    for v in vv {
                        rrss[v] = i + 1;
                    }
                    rrss[p] = i + 1;
                } else {
                    vv.push(p);
                    btm.insert(p, vv);
                    // eprintln!("update {q}->{p}");
                }
            } else {
                btm.insert(p, vec![p]);
                // eprintln!("isnert {p}");
            }
        }
    }
    for rs in rrss {
        if rs == 0 {
            println!("-1");
        } else {
            println!("{rs}");
        }
    }
}