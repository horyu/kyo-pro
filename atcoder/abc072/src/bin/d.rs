#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
    };
    let mut rs = 0;
    // 連続した要素が index と被っていたら +1
    // 1個だけ index と同じでも +1
    let mut i = 0;
    while i < n {
        if i == pp[i] {
            rs += 1;
            if (i + 1 < n) && ((i + 1) == pp[i + 1]) {
                i += 1;
            }
        }
        i += 1;
    }
    println!("{}", rs);
}
