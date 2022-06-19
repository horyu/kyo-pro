#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [isize; n],
        llrrvv: [(Usize1, Usize1, isize); q]
    };
    if n == 1 {
        print!("{}", "0\n".to_string().repeat(q));
        return;
    }
    let mut dd = vec![0];
    let mut e = 0;
    for (ai, aj) in aa.iter().tuple_windows() {
        dd.push(aj - ai);
        e += (aj - ai).abs();
    }
    dd.push(0);
    for (l, r, v) in llrrvv {
        let mae = dd[l].abs() + dd[r + 1].abs();
        if l > 0 {
            dd[l] += v;
        }
        if r < n - 1 {
            dd[r + 1] -= v;
        }
        let ato = dd[l].abs() + dd[r + 1].abs();
        e += ato - mae;
        println!("{e}");
    }
}
