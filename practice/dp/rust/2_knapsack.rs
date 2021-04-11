use std::cmp::max;

struct Item {
    weight: usize,
    value: usize,
}

fn main() {
    let n: usize = 6;
    let items: Vec<Item> = vec![
        Item {
            weight: 2,
            value: 3,
        },
        Item {
            weight: 1,
            value: 2,
        },
        Item {
            weight: 3,
            value: 6,
        },
        Item {
            weight: 2,
            value: 1,
        },
        Item {
            weight: 1,
            value: 3,
        },
        Item {
            weight: 5,
            value: 85,
        },
    ];
    let w: usize = 9;

    let mut dp = vec![vec![0usize; w + 1]; n + 1];

    for (i, item) in items.iter().enumerate() {
        for w in 0..(w + 1) {
            if w >= item.weight {
                dp[i + 1][w] = max(dp[i][w], dp[i][w - item.weight] + item.value);
            } else {
                dp[i + 1][w] = dp[i][w];
            }
        }
    }

    println!("{}", dp[n][w])
}
