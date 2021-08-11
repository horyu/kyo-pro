#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    if n == 1 {
        println!("{}", aa[0] ^ bb[0]);
        return;
    }
    let mut hm = HashMap::new();
    for &b in &bb {
        *hm.entry(b).or_insert(0) += 1;
    }
    let mut oks = HashSet::new();
    for &b in bb.iter() {
        let xor = aa[0] ^ b;
        if oks.contains(&xor) {
            continue;
        }
        let mut b_cnts = hm.clone();
        *b_cnts.entry(b).or_default() -= 1;
        let is_ok = aa[1..].iter().all(|a| {
            let v = xor ^ a;
            if let Some(b_cnt) = b_cnts.get_mut(&v) {
                if *b_cnt > 0 {
                    *b_cnt -= 1;
                    return true;
                }
            }
            false
        });
        if is_ok {
            oks.insert(xor);
        }
    }
    let len = oks.len();
    if len == 0 {
        println!("0");
    } else {
        let oks = oks.iter().sorted().map(|ok| ok.to_string()).join("\n");
        println!("{}\n{}", len, oks);
    }
}
