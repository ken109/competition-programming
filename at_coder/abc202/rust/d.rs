use proconio::input;

fn ncr(n: u64, r: u64) -> u64 {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => ncr(n, r - 1) * (n - r + 1) / r,
    }
}

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
        k: u64,
    }

    let mut pass: u64 = 0;
    let mut ans = "".to_string();

    for _ in 0..(a + b) {
        let a_s = ncr(a + b - 1, b);

        if k <= a_s + pass && a > 0 {
            ans.push('a');
            a -= 1;
        } else {
            ans.push('b');
            b -= 1;
            pass += a_s;
        }
    }

    println!("{}", ans);
}
