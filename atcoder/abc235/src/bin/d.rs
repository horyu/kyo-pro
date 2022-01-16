#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
#![feature(int_log)]
use itertools::Itertools;
use num_integer::*;
use proconio::{fastout, input, marker::*};
use std::collections::*;

#[fastout]
fn main() {
    input! {
        a: usize,
        n: usize,
    };
    let mut i = 1;
    let mut viewed = HashSet::new();
    let mut qq = VecDeque::new();
    let upper = 10usize.pow(n.log10() as u32 + 1);
    qq.push_back(1usize);
    viewed.insert(1);
    while !qq.is_empty() {
        let mut next_qq = VecDeque::new();
        while let Some(q) = qq.pop_front() {
            let x = q * a;
            if x == n {
                println!("{}", i);
                return;
            }
            if !viewed.contains(&x) && x < upper {
                viewed.insert(x);
                next_qq.push_back(x);
            }

            let digit = q.log10() as u32;
            let tail = q % 10;
            if digit > 0 && tail != 0 {
                let y = q / 10 + tail * 10usize.pow(digit);
                if y == n {
                    println!("{}", i);
                    return;
                }
                if !viewed.contains(&y) && y < upper {
                    viewed.insert(y);
                    next_qq.push_back(y);
                }
            }
        }
        qq = next_qq;
        i += 1;
    }

    println!("-1");
}
