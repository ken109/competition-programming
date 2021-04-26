use proconio::input;

fn main() {
    input! {
        n: u32,
        av: [i32; n],
        bv: [i32; n],
    }

    let ans = bv.iter().min().unwrap() - av.iter().max().unwrap();

    if 0 <= ans {
        println!("{}", ans + 1)
    } else {
        println!("0")
    }
}
