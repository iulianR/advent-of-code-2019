use itertools::Itertools;

static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

struct Program {
    ops: Vec<i32>,
    ip: usize,
    phase: i32,
}

impl Program {
    fn new(ops: Vec<i32>, phase: i32) -> Self {
        Program { ops, ip: 0, phase }
    }

    fn op(&self) -> i32 {
        self.ops[self.ip] % 100
    }

    fn param(&self, arg: usize) -> i32 {
        if self.ops[self.ip] / 10_i32.pow(arg as u32 + 1) % 10 == 0 {
            self.ops[self.ops[arg + self.ip] as usize]
        } else {
            self.ops[arg + self.ip]
        }
    }

    fn step(&mut self, value: usize) {
        self.ip += value
    }

    fn set(&mut self, arg: usize, value: i32) {
        let index = self.ops[self.ip + arg] as usize;
        self.ops[index] = value
    }

    fn jump(&mut self, arg: usize) {
        self.ip = arg
    }

    fn get(&self, arg: usize) -> i32 {
        self.ops[arg]
    }

    fn run(&mut self, input: i32) -> Option<i32> {
        loop {
            let op = self.op().into();
            match op {
                Op::Add => {
                    self.set(3, self.param(1) + self.param(2));
                    self.step(4)
                }
                Op::Multiply => {
                    self.set(3, self.param(1) * self.param(2));
                    self.step(4)
                }
                Op::Input => {
                    if self.phase != -1 {
                        self.set(1, self.phase);
                        self.phase = -1;
                    } else {
                        self.set(1, input);
                    }
                    self.step(2)
                }
                Op::Output => {
                    let out = self.param(1);
                    self.step(2);
                    return Some(out);
                }
                Op::JumpIfTrue => {
                    if self.param(1) != 0 {
                        self.jump(self.param(2) as usize)
                    } else {
                        self.step(3)
                    }
                }
                Op::JumpIfFalse => {
                    if self.param(1) == 0 {
                        self.jump(self.param(2) as usize)
                    } else {
                        self.step(3)
                    }
                }
                Op::LessThan => {
                    if self.param(1) < self.param(2) {
                        self.set(3, 1)
                    } else {
                        self.set(3, 0)
                    }
                    self.step(4)
                }
                Op::Equals => {
                    if self.param(1) == self.param(2) {
                        self.set(3, 1)
                    } else {
                        self.set(3, 0)
                    }
                    self.step(4)
                }
                Op::Stop => {
                    return None;
                }
            }
        }
    }
}
enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Stop,
}

impl From<i32> for Op {
    fn from(item: i32) -> Self {
        use Op::*;

        match item {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            99 => Stop,
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<()> {
    let ops: Result<Vec<i32>, _> = INPUT.trim().split(',').map(str::parse::<i32>).collect();
    let ops = ops?;

    let part1 = (0..=4)
        .permutations(5)
        .map(|inputs| {
            let mut output = 0;

            let mut programs = vec![];
            for i in inputs {
                programs.push(Program::new(ops.clone(), i));
            }

            for program in programs.iter_mut().take(4 + 1) {
                match program.run(output) {
                    Some(out) => output = out,
                    None => break,
                }
            }

            output
        })
        .max()
        .unwrap();

    let part2 = (5..=9)
        .permutations(5)
        .map(|inputs| {
            let mut prev_output = 0;

            let mut programs = vec![];
            for i in inputs.iter() {
                programs.push(Program::new(ops.clone(), *i));
            }

            'outer: loop {
                for program in programs.iter_mut().take(4 + 1) {
                    match program.run(prev_output) {
                        Some(output) => prev_output = output,
                        None => break 'outer,
                    }
                }
            }
            prev_output
        })
        .max()
        .unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
