use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        members: [[usize; 5]; n]
    }

    let check = |m: usize| -> bool {
        let mut s = HashSet::<usize>::new();

        for member in members.iter() {
            s.insert(
                (0..member.len())
                    .filter(|v| member[*v as usize] >= m)
                    .map(|v| 1 << v)
                    .sum(),
            );
        }

        for x in s.clone().iter() {
            for y in s.clone().iter() {
                for z in s.clone().iter() {
                    if x | y | z == 31 {
                        return true;
                    }
                }
            }
        }
        return false;
    };

    let mut ok: usize = 0;
    let mut ng = 10usize.pow(9) + 1;

    while ng - ok > 1 {
        let m = (ok + ng) / 2;
        if check(m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    println!("{:?}", ok)
}
