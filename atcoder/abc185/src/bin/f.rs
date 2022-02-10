#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [usize; n],
        ttxxyy: [(u8, usize, usize); q]
    };
    let mut st = SegmentTree::new(n, 0, |a, b| a ^ b);
    for (i, a) in aa.into_iter().enumerate() {
        st.update(i, a);
    }
    for (t, x, y) in ttxxyy {
        if t == 1 {
            st.update(x - 1, y);
        } else {
            println!("{}", st.get(x - 1, y - 1));
        }
    }
}

// https://easthop.hatenablog.com/entry/2020/12/15/211044
struct SegmentTree<F, T> {
    size: usize,
    tree: Vec<T>,
    element: T,
    eval: F,
}

impl<F: Fn(T, T) -> T, T: Copy + Eq + std::fmt::Debug> SegmentTree<F, T> {
    fn new(max: usize, element: T, eval: F) -> Self {
        // サイズを収まる範囲の 2^x乗 にする
        let size = max.next_power_of_two();
        Self {
            size,
            tree: vec![element; size * 2], // セグ木はその2倍のサイズ
            element,
            eval,
        }
    }

    // 開閉区間のの値を取得する
    // new した時のロジックで処理される
    fn get(&self, left: usize, right: usize) -> T {
        self._get(left, right + 1, 1, 0, self.size)
    }

    fn _get(&self, left: usize, right: usize, now_pos: usize, l: usize, r: usize) -> T {
        // 捜索範囲を超えた場合 初期値 を返す
        if r <= left || right <= l {
            self.element
        // 探索終了条件
        // 二分探索して値を見つけた場合
        } else if left <= l && r <= right {
            self.tree[now_pos]
        // 探索が続く場合
        // 今のポジションから左(*2)と右(*2+1)に移動
        // 左に移動した場合、右端をずらす
        // 右に移動した場合、左端をずらす
        } else {
            (self.eval)(
                self._get(left, right, now_pos * 2, l, (l + r) / 2), // 左
                self._get(left, right, now_pos * 2 + 1, (l + r) / 2, r), //右
            )
        }
    }

    pub fn update(&mut self, index: usize, value: T) {
        let mut i = self.size + index;
        while i != 0 {
            let before = self.tree[i];
            let after = (self.eval)(before, value);

            // 更新しても変わらない場合その後も変わらない
            if before == after {
                break;
            }
            self.tree[i] = after;
            i /= 2;
        }
    }
}
