#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut vse: [String; n]
    };
    // 00:00-24:00 を5分刻み 60/5*24
    // 0 -> 0000-0005, 1 => 0005-0010, ...
    let mut arr = [false; 60 / 5 * 24 + 1];
    for se in vse {
        let si = s_to_index(&se[0..=3]);
        let ei = e_to_index(&se[5..=8]);
        // dbg!(&se[0..=3], index_to_time(si));
        // dbg!(&se[5..=8], index_to_time(ei));
        for i in si..ei {
            arr[i] = true;
        }
    }
    // for (i, &tf) in arr.iter().enumerate() {
    //     print!("{}", if tf {
    //         "T "
    //     } else {
    //         "F "
    //     });
    //     if i % 12 == 11 {
    //         println!("");
    //     }
    // }
    let mut pre = false;
    for (i, &now) in arr.iter().enumerate() {
        match (pre, now) {
            (true, true) => {
                pre = true;
            }
            (true, false) => {
                println!("{}", index_to_time(i));
                pre = false;
            }
            (false, true) => {
                print!("{}-", index_to_time(i));
                pre = true;
            }
            (false, false) => {
                pre = false;
            }
        };
    }
    if pre {
        println!("2400");
    }
}

fn s_to_index(hhmm: &str) -> usize {
    let hh = &hhmm[0..=1];
    let mm = &hhmm[2..=3];
    to_usize(hh) * 12 + (to_usize(mm) / 5)
}

fn e_to_index(hhmm: &str) -> usize {
    let hh = &hhmm[0..=1];
    let mm = &hhmm[2..=3];
    let tmp = to_usize(mm);
    if tmp % 5 == 0 {
        to_usize(hh) * 12 + (tmp / 5)
    } else {
        to_usize(hh) * 12 + (tmp / 5) + 1
    }
}

fn to_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn index_to_time(i: usize) -> String {
    let hh = i / 12;
    let mm = i % 12 * 5;
    format!("{:02}{:02}", hh, mm)
}
