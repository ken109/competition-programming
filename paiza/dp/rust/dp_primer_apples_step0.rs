fn main() {
    let n = 5;
    let a = 100;
    let b = 150;

    let mut dp = vec![0; n + 1];

    dp[0] = 0;
    dp[1] = a;

    for i in 2..(n + 1) {
        dp[i] = (dp[i - 1] + a).min(dp[i - 2] + b)
    }

    println!("{:?}", dp);
}
