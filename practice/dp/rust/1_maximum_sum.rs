use std::cmp::max;

fn main() {
    let n: usize = 3;
    let a: Vec<i32> = vec![7, -6, 9];

    let mut dp: Vec<i32> = vec![0; n + 1];

    for (i, v) in a.iter().enumerate() {
        dp[i + 1] = max(dp[i], dp[i] + *v)
    }

    println!("{}", dp[n])
}
