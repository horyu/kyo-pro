#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: i32,
    };
    let mut count = 0;
    for ori_i in 1..std::usize::MAX {
        let mut i = ori_i;
        let tmp = i % 10;
        let tf = loop {
            i /= 10;
            if i == 0 {
                break true;
            }
            if i % 10 != tmp {
                break false;
            }
        };
        if tf {
            count += 1;
            if count == n {
                println!("{}", ori_i);
                return;
            }
        }
    }
}
