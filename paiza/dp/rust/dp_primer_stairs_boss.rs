fn main() {
    let n = 10;
    let a = 2;
    let b = 3;
    let c = 4;

    let mut dp = vec![0; n + 1];

    dp[0] = 1;

    for i in 0..(n + 1) {
        if i as i32 >= a {
            dp[i] += dp[i - a as usize];
        }
        if i as i32 >= b {
            dp[i] += dp[i - b as usize];
        }
        if i as i32 >= c {
            dp[i] += dp[i - c as usize];
        }
    }

    println!("{:?}", dp);
}
