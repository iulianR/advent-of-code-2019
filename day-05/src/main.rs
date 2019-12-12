use std::io::{Read};
use std::io;

static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

struct Program<'a> {
    ops: &'a mut [i32],
    ip: usize,
}

impl<'a> Program<'a> {
    fn new(ops: &'a mut [i32]) -> Self {
        Program { ops, ip: 0 }
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

    fn run(&mut self, input: i32) {
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
                    self.set(1, input);
                    self.step(2)
                }
                Op::Output => {
                    if self.param(1) != 0 {
                        println!("{} ", self.param(1));
                    }
                    self.step(2)
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
                    return;
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
    let mut ops = ops?;

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let input = buf.trim().parse::<i32>().unwrap();
    let mut program = Program::new(&mut ops);
    program.run(input as i32);

    Ok(())
}
