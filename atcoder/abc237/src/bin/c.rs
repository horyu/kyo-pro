#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        ss: Chars
    };
    let mut l = 0;
    let mut r = ss.len() - 1;
    while l < r {
        if ss[r] == 'a' {
            r -= 1;
            if ss[l] == 'a' {
                l += 1;
            }
            continue;
        }
        break;
    }
    while l < r {
        let sl = ss[l];
        let sr = ss[r];
        if sl != sr {
            println!("No");
            return;
        }
        l += 1;
        r -= 1;
    }
    println!("Yes");
}
