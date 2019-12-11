static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
struct Instruction {
    dir: char,
    distance: i32,
}

impl Instruction {
    fn new(d: char, distance: i32) -> Self {
        Instruction { dir: d, distance }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

fn build_path(instructions: &[Instruction], path: &mut Vec<Position>) {
    path.push(Position::new(0, 0));
    let mut latest = Position::new(0, 0);
    for i in instructions {
        let l = latest;
        match i.dir {
            'R' => {
                for pos in l.x..(l.x + i.distance) {
                    path.push(Position::new(pos, l.y));
                }
                latest = Position::new(l.x + i.distance, l.y);
            }
            'L' => {
                for pos in ((l.x - i.distance)..l.x).rev() {
                    path.push(Position::new(pos, l.y));
                }
                latest = Position::new(l.x - i.distance, l.y);
            }
            'U' => {
                for pos in l.y..(l.y + i.distance) {
                    path.push(Position::new(l.x, pos));
                }
                latest = Position::new(l.x, l.y + i.distance);
            }
            'D' => {
                for pos in ((l.y - i.distance)..l.y).rev() {
                    path.push(Position::new(l.x, pos));
                }
                latest = Position::new(l.x, l.y - i.distance);
            }
            _ => unimplemented!(),
        }
    }
}

fn main() -> Result<()> {
    let v: Vec<&str> = INPUT.lines().collect();
    let first: Vec<Instruction> = v[0]
        .split(',')
        .map(|x| Instruction::new(x.chars().nth(0).unwrap(), x[1..].parse::<i32>().unwrap()))
        .collect();
    let second: Vec<Instruction> = v[1]
        .split(',')
        .map(|x| Instruction::new(x.chars().nth(0).unwrap(), x[1..].parse::<i32>().unwrap()))
        .collect();

    let mut path1 = vec![];
    build_path(&first, &mut path1);

    let mut path2 = vec![];
    build_path(&second, &mut path2);

    let path1_clone = path1.clone();
    let path2_clone = path2.clone();

    path2.sort();

    let min = path1
        .clone()
        .into_iter()
        .skip_while(|p| p.x == 0 && p.y == 0)
        .filter(|p| path2.binary_search(&p).is_ok())
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .unwrap_or(0);
    println!("Part one: {}", min);

    let min = path1
        .into_iter()
        .skip_while(|p| p.x == 0 && p.y == 0)
        .filter(|p| path2.binary_search(&p).is_ok())
        .map(|p| {
            path1_clone.iter().position(|x| *x == p).unwrap()
                + path2_clone.iter().position(|x| *x == p).unwrap()
        })
        .min()
        .unwrap_or(0);
    println!("Part two: {}", min);


    Ok(())
}
