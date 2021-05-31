use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug)]
struct F {
    i: u64,
    m: u64,
}

fn main() {
    input! {
        (n, k): (u64, u64),
        mut fs: [F; n]
    }

    fs.sort_by(|a, b| a.i.cmp(&b.i));

    let mut now: u64 = 0;
    let mut remain: u64 = k;

    for f in fs {
        if now < f.i && f.i - now > remain {
            break;
        }
        remain -= f.i - now;
        now = f.i;
        remain += f.m;
    }

    println!("{}", now + remain);
}
