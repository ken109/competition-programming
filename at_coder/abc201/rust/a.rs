use proconio::input;

fn main() {
    input! {
        mut n: [i64; 3]
    }

    n.sort();

    if n[2] - n[1] == n[1] - n[0] {
        println!("Yes")
    } else {
        println!("No")
    }
}
