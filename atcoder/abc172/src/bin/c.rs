#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u128,
        aa: [u128; n],
        bb: [u128; m],
    };
    // n,m <= 200_000
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    // aa優先で読めるだけ読みbも読む
    while i < n {
        let next_sum = sum + aa[i];
        if next_sum <= k {
            sum = next_sum;
            i += 1;
            continue;
        }
        break;
    }
    while j < m {
        let next_sum = sum + bb[j];
        if next_sum <= k {
            sum = next_sum;
            j += 1;
            continue;
        }
        break;
    }
    let mut rs = i + j;
    // bbの読む数を増やしていく→aaを減らす
    while i > 0 && j < m {
        i -= 1;
        sum -= aa[i];
        while j < m {
            let next_sum = sum + bb[j];
            if next_sum <= k {
                sum = next_sum;
                j += 1;
                continue;
            }
            break;
        }
        rs = rs.max(i + j);
    }
    println!("{rs}");
}
