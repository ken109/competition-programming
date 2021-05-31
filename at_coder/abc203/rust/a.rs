use proconio::input;

fn main() {
    input! {
        (a,b,c): (usize, usize, usize)
    }

    if a == b {
        println!("{}", c)
    } else if b == c {
        println!("{}", a)
    } else if a == c {
        println!("{}", b)
    } else {
        println!("0")
    }
}
