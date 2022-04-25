#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        t: usize,
        aabbcc: [(usize, usize, usize); t]
    };
    for (mut a, mut b, mut c) in aabbcc {
        b /= 2; // 長さ6にする
        let mut rs = 0;
        let bc = b.min(c);
        b -= bc;
        c -= bc;
        rs += bc;
        // eprintln!("{rs} {a} {b} {c}");
        match (b > 0, c > 0) {
            (true, false) => {
                let aab = (a / 2).min(b);
                a -= aab * 2;
                // b -= aab;
                rs += aab;
                // eprintln!("{rs} {a} {b} {c}");
            }
            (false, true) => {
                let acc = a.min(c / 2);
                a -= acc;
                c -= acc * 2;
                rs += acc;
                // eprintln!("{rs} {a} {b} {c}");
                let aaac = (a / 3).min(c);
                a -= aaac * 3;
                // c -= aaac;
                rs += aaac;
                // eprintln!("{rs} {a} {b} {c}");
            }
            _ => (),
        }
        rs += a / 5;
        // eprintln!("{rs} {a} {b} {c}");
        println!("{rs}");
    }
}
