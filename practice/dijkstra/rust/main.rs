use std::collections::HashMap;

struct Point {
    id: char,
    to: Vec<(char, usize)>,
}

fn main() {
    let points = vec![
        Point {
            id: 'a',
            to: vec![('b', 1), ('c', 4)],
        },
        Point {
            id: 'b',
            to: vec![('c', 2), ('d', 5)],
        },
        Point {
            id: 'c',
            to: vec![('b', 1), ('d', 1)],
        },
        Point {
            id: 'd',
            to: vec![],
        },
    ];

    let mut prevs = HashMap::<char, char>::new();
    let mut cost = HashMap::<char, usize>::new();
    let mut pending = HashMap::<char, usize>::new();

    // initialize
    for point in points.iter() {
        pending.insert(point.id, usize::MAX);
    }

    pending.insert(points[0].id, 0);

    // calculate
    while pending.iter().count() > 0 {
        let (&min_id, &min_cost) = pending.iter().min().unwrap();

        cost.insert(min_id, min_cost);

        pending.remove(&min_id);

        for (to_id, to_cost) in &points.iter().find(|&p| p.id == min_id).unwrap().to {
            let total_cost = cost.get(&min_id).unwrap() + to_cost;

            if pending.contains_key(to_id) && pending.get(to_id).unwrap() > &total_cost {
                pending.insert(*to_id, total_cost);
                prevs.insert(*to_id, min_id);
            }
        }
    }

    println!("{:?}", prevs);
    println!("{:?}", cost);
}
