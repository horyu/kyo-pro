#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::VecDeque;

fn main() {
    input! {
        r: usize,
        _c: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        ccc: [Chars; r],
    };
    // # : std::isize::MIN
    // . : -1
    // s : 0
    // g : std::isize::MAX
    let mut ccc = ccc
        .iter()
        .map(|cc| {
            cc.iter()
                .map(|&c| if c == '#' { std::isize::MIN } else { -1isize })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    ccc[s.0][s.1] = 0;
    ccc[g.0][g.1] = std::isize::MAX;
    let mut queue = VecDeque::new();
    queue.push_front(s);
    while !queue.is_empty() {
        let center = queue.pop_front().unwrap();
        let next_num = ccc[center.0][center.1] + 1;
        let xy_udrl = vec![
            (center.0, center.1 - 1),
            (center.0, center.1 + 1),
            (center.0 + 1, center.1),
            (center.0 - 1, center.1),
        ];
        for xy in xy_udrl {
            let num = ccc[xy.0][xy.1];
            if num == std::isize::MAX {
                println!("{}", next_num);
                return;
            }
            if num == -1 {
                ccc[xy.0][xy.1] = next_num;
                queue.push_back(xy);
            }
        }
    }
}
