#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize
    };
    if n <= 2 {
        println!("{}", (1..=n).join(" "));
        return;
    }
    let mut aa = vec![1, 2];
    for i in 3..=n {
        let next = {
            // 素因数の数 + 1
            let mut num = i;
            let mut cnt = 0;
            let mut j = 1usize;
            while j * j <= num {
                j += 1;
                if num % j != 0 {
                    continue;
                }
                while num % j == 0 {
                    cnt += 1;
                    num /= j;
                }
            }
            if num != 1 {
                cnt += 1;
            }
            cnt + 1
        };
        aa.push(next);
    }
    // eprintln!("{}", (1..=n).join("\t"));
    // eprintln!("{}", aa.iter().join("\t"));
    println!("{}", aa.iter().join(" "));
}
