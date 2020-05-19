#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        t: usize,
        n: usize,
        mut aa: [usize; n],
        m: usize,
        mut bb: [usize; m]
    };
    for _i in 0..m {
        let b = bb.remove(0);
        loop {
            if aa.is_empty() {
                println!("no");
                return;
            }
            let a = aa.remove(0);
            if (a <= b) && (a + t >= b) {
                break;
            }
        }
    }
    println!("yes");
}
