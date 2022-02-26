#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
        bb: [usize; m],
    };
    let mut hm = HashMap::new();
    for a in aa {
        *hm.entry(a).or_insert(0) += 1;
    }
    for b in bb {
        if let Some(e) = hm.get_mut(&b) {
            *e -= 1;
            if *e == 0 {
                hm.remove(&b);
            }
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
