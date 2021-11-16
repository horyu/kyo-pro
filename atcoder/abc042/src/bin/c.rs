#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut n: usize,
        k: usize,
        dd: [usize; k]
    };
    let outs = {
        let mut arr = [false; 10];
        for d in dd {
            arr[d] = true;
        }
        arr
    };
    loop {
        if all_ok(&outs, n) {
            println!("{}", n);
            return;
        }
        n += 1;
    }
}

fn all_ok(outs: &[bool; 10], mut n: usize) -> bool {
    while n != 0 {
        if outs[n % 10] {
            return false;
        }
        n /= 10;
    }
    true
}
