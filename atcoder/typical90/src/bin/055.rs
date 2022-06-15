#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        aa: [usize; n]
    };
    let mut rs = 0usize;
    let len = aa.len();
    for i0 in 0..len {
        let b0 = aa[i0] % p;
        for i1 in (i0 + 1)..len {
            let b1 = (b0 * aa[i1]) % p;
            for i2 in (i1 + 1)..len {
                let b2 = (b1 * aa[i2]) % p;
                for i3 in (i2 + 1)..len {
                    let b3 = (b2 * aa[i3]) % p;
                    for i4 in (i3 + 1)..len {
                        if (b3 * aa[i4]) % p == q {
                            rs += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{rs}");
}
