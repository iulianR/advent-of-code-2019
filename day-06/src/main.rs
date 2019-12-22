use std::collections::{HashMap, VecDeque};

static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn bfs<'a>(orbits: &HashMap<&str, &'a str>, source: &'a str, path: &mut Vec<&'a str>) {
    let mut q = VecDeque::new();
    q.push_back(source);

    while !q.is_empty() {
        let curr = q.pop_front().unwrap();
        path.push(&curr);
        if curr == "COM" {
            return;
        }

        q.push_back(&orbits.get(curr).unwrap());
    }
}

fn main() -> Result<()> {
    let orbits = INPUT
        .lines()
        .map(|l| {
            let mut pair = l.split(')');
            let first = pair.next().unwrap();
            let last = pair.next().unwrap();
            (last, first)
        })
        .collect::<HashMap<_, _>>();

    let mut count = 0;
    for mut orbiting_body in orbits.keys() {
        while let Some(orbited_body) = orbits.get(orbiting_body) {
            count += 1;
            orbiting_body = orbited_body;
        }
    }

    let mut path1 = vec![];
    bfs(&orbits, &orbits.get("YOU").unwrap(), &mut path1);

    let mut path2 = vec![];
    bfs(&orbits, &orbits.get("SAN").unwrap(), &mut path2);

    path1.reverse();
    path2.reverse();

    if path1.len() < path2.len() {
        path1.resize(path2.len(), "");
    } else {
        path2.resize(path1.len(), "");
    }

    let mut minpath = 0;
    let _ : Vec<_>= path1
        .iter()
        .zip(path2.iter())
        .filter(|(&p1, &p2)| p1 != p2)
        .map(|(&p1, &p2)| {
            if p1 != "" {
                minpath +=1;
            }

            if p2 != "" {
                minpath += 1;
            }
        })
        .collect();

    println!("Part 1: {}", count);
    println!("Part 2: {}", minpath);
    Ok(())
}
