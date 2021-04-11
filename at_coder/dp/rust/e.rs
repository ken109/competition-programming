use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        (n, w): (usize, usize),
        wv: [(usize, usize); n]
    }

    let mut dp = vec![vec![2 << 60 as usize; 100001]; n + 1];

    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..=100000 {
            if j >= wv[i - 1].1 {
                dp[i][j] = min(dp[i][j], dp[i - 1][j - wv[i - 1].1] + wv[i - 1].0);
            }
            dp[i][j] = min(dp[i][j], dp[i - 1][j]);
        }
    }

    println!("{}", (0..=100000).rev().find(|i| dp[n][*i] <= w).unwrap());
}
