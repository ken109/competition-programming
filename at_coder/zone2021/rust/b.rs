use proconio::input;

fn main() {
    input! {
        n: usize,
        ud: f64,
        uh: f64,
        bs: [(f64, f64); n]
    }

    let ans = bs
        .iter()
        .map(|(bd, bh)| uh - ud * ((uh - bh) / (ud - bd)))
        .fold(0. / 0., f64::max);

    println!("{}", if ans < 0. { 0. } else { ans });
}
