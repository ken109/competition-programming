use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut sum: usize = 0;
    for i in 1..=n {
        for j in 1..=k {
            sum += (i.to_string() + "0" + &*j.to_string())
                .parse::<usize>()
                .unwrap()
        }
    }

    println!("{}", sum)
}
