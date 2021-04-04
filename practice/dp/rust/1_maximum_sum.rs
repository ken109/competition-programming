use std::cmp::max;

fn main() {
    let n: i32 = 3;
    let a: Vec<i32> = vec![7, -6, 9];

    let mut dp: Vec<i32> = vec![0; (n + 1) as usize];

    for i in 0..a.len() {
        dp[i + 1] = max(dp[i], dp[i] + a[i])
    }

    println!("{}", dp[n as usize])
}
