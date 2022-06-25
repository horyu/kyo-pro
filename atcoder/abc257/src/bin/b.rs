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
        q: usize,
        aa: [Usize1; k],
        ll: [Usize1; q],
    };
    let mut koma2masu = aa;
    let mut masu2koma = vec![std::usize::MAX; n];
    for (i, &a) in koma2masu.iter().enumerate() {
        masu2koma[a] = i;
    }
    for l in ll {
        let masu = koma2masu[l];
        if masu == n - 1 {
            continue;
        }
        if masu2koma[masu + 1] == std::usize::MAX {
            koma2masu[l] += 1;
            masu2koma.swap(masu, masu + 1);
        }
    }
    for rs in koma2masu {
        println!("{}", rs + 1);
    }
}
