#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut s1: Chars,
        mut s2: Chars,
        mut s3: Chars,
    };
    let mut hs = HashSet::new();
    if s1.len().max(s2.len()) > s3.len() {
        println!("UNSOLVABLE");
        return;
    }
    for &c in s1.iter().chain(s2.iter()).chain(s3.iter()) {
        hs.insert(c);
    }
    if hs.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }
    // 数字にしやすいように逆順に
    s1.reverse();
    s2.reverse();
    s3.reverse();
    let chars = hs.into_iter().collect_vec();
    // Sの先頭（逆順なのでlast）に0は割り当てられない
    let lasts = vec![s1.last().unwrap(), s2.last().unwrap(), s3.last().unwrap()];
    // hs の各文字に重複しないように 0~9 を割り当てて計算する
    let rs = (0..=9).permutations(chars.len()).find(|nums| {
        if lasts
            .iter()
            .any(|&last| nums[chars.iter().position(|c| c == last).unwrap()] == 0)
        {
            return false;
        }
        let s1_num = to_num(&s1, nums, &chars);
        let s2_num = to_num(&s2, nums, &chars);
        let s3_num = to_num(&s3, nums, &chars);
        s1_num + s2_num == s3_num
    });
    if let Some(nums) = rs {
        for s in vec![s1, s2, s3] {
            let s_num = to_num(&s, &nums, &chars);
            println!("{}", s_num);
        }
    } else {
        println!("UNSOLVABLE");
    }
}

fn to_num(s: &Vec<char>, nums: &Vec<isize>, chars: &Vec<char>) -> isize {
    let mut rs = 0;
    for (i, sc) in s.iter().enumerate() {
        rs += 10isize.pow(i as u32) * nums[chars.iter().position(|c| c == sc).unwrap()];
    }
    rs
}
