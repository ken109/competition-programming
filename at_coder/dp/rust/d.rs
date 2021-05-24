use proconio::input;

fn main() {
    input! {
        (n, w): (usize, usize),
        wv: [(usize, usize); n]
    }

    let mut dp = vec![vec![0 as usize; w + 1]; n + 1];

    for (i, (weight, value)) in wv.iter().enumerate() {
        for w in 0..=w {
            if w >= *weight {
                dp[i + 1][w] = dp[i][w].max(dp[i][w - *weight] + *value);
            } else {
                dp[i + 1][w] = dp[i][w];
            }
        }
    }

    println!("{}", dp[n][w])
}
