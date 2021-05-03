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

    let mut parent = HashMap::<char, char>::new();
    let mut distance = HashMap::<char, usize>::new();
    let mut heap = HashMap::<char, usize>::new();

    for point in points.iter() {
        heap.insert(point.id, usize::MAX);
    }

    heap.insert(points[0].id, 0);

    while heap.iter().count() > 0 {
        let (&min_id, &min_distance) = heap.iter().min().unwrap();

        distance.insert(min_id, min_distance);

        heap.remove(&min_id.clone());

        for (to_id, to_distance) in &points.iter().find(|&p| p.id == min_id.clone()).unwrap().to {
            let now_distance = distance.get(&min_id).unwrap() + to_distance;

            if heap.contains_key(to_id) && heap.get(to_id).unwrap() > &now_distance {
                heap.insert(*to_id, now_distance);
                parent.insert(*to_id, min_id);
            }
        }
    }

    println!("{:?}", parent);
    println!("{:?}", distance);
}
