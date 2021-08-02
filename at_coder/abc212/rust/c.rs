use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
        mut a: [u64; n],
        mut b: [u64; m]
    }

    a.sort();
    b.sort();

    if a[0] > *b.last().unwrap() {
        println!("{}", a[0] - *b.last().unwrap())
    } else if b[0] > *a.last().unwrap() {
        println!("{}", b[0] - *a.last().unwrap())
    } else {
        let mut ans = std::i64::MAX;
        let mut x: u64 = 0;
        let mut y: u64 = 0;

        while (x < n) && (y < m) {
            ans = ans.min((a[x as usize] as i64 - b[y as usize] as i64).abs());
            if a[x as usize] > b[y as usize] {
                y += 1;
            } else {
                x += 1;
            }
        }
        println!("{}", ans);
    }
}
