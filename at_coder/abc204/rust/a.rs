use proconio::input;

fn main() {
    input! {x: usize, y: usize};

    if x != y {
        let mut a: Vec<usize> = vec![0, 1, 2];
        a = a.into_iter().filter(|&v| v != x && v != y).collect();
        println!("{}", a[0]);
    } else {
        println!("{}", x);
    }
}
