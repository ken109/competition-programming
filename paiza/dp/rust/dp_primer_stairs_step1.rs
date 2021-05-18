// https://paiza.jp/works/mondai/dp_primer/dp_primer_stairs_step1

fn main() {
    let n = 11;
    let a = 3;
    let b = 4;

    let mut dp = vec![0; n + 1];

    dp[0] = 1;

    for i in 0..(n + 1) {
        if i as i32 >= a {
            dp[i] += dp[i - a as usize];
        }
        if i as i32 >= b {
            dp[i] += dp[i - b as usize];
        }
    }

    println!("{:?}", dp);
}
