use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug)]
struct Mount {
    name: String,
    height: i64,
}

fn main() {
    input! {
        n: u64,
        mut ms: [Mount; n]
    }

    ms.sort_by(|f, s| s.height.cmp(&f.height));

    println!("{}", ms[1].name)
}
