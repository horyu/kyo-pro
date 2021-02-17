#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    };
    let mut v = vec![false; 1_000_000_000];
    for a in aa {
        if v[a] {
            println!("NO");
            return;
        }
        v[a] = true;
    }
    println!("YES");
}
