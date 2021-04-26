use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        qn: usize,
        qs: [(usize, usize, usize); qn]
    }

    let mut rev = false;

    let get_index = |rev: bool, v: usize| -> usize {
        if !rev {
            v - 1
        } else {
            if v <= n {
                v + n - 1
            } else {
                v - n - 1
            }
        }
    };

    for q in qs.iter() {
        if q.0 == 1 {
            let a = get_index(rev, q.1);
            let b = get_index(rev, q.2);
            let tmp = s[a];
            s[a] = s[b];
            s[b] = tmp;
        } else {
            rev = !rev;
        }
    }

    if rev {
        println!(
            "{}{}",
            Vec::from(&s[n..]).iter().collect::<String>(),
            Vec::from(&s[..n]).iter().collect::<String>(),
        )
    } else {
        println!("{}", s.iter().collect::<String>())
    }
}
