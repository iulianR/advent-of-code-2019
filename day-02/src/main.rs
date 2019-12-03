static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

enum Op {
    Add,
    Multiply,
    Stop,
}

impl From<usize> for Op {
    fn from(item: usize) -> Self {
        use Op::*;

        match item {
            1 => Add,
            2 => Multiply,
            99 => Stop,
            _ => unreachable!(),
        }
    }
}

fn run_program(program: &[usize], noun: Option<usize>, verb: Option<usize>) -> usize {
    let mut program = program.to_owned();

    if let Some(noun) = noun {
        let verb = verb.unwrap();
        program[1] = noun;
        program[2] = verb;
    }

    let mut i = 0usize;
    loop {
        let op = program[i].into();
        match op {
            Op::Add => {
                let pos_input_1 = program[i + 1];
                let pos_input_2 = program[i + 2];
                let pos_result = program[i + 3];
                program[pos_result] = program[pos_input_1] + program[pos_input_2];
                i += 4;
            }
            Op::Multiply => {
                let pos_input_1 = program[i + 1];
                let pos_input_2 = program[i + 2];
                let pos_result = program[i + 3];
                program[pos_result] = program[pos_input_1] * program[pos_input_2];
                i += 4;
            }
            Op::Stop => return program[0],
        }
    }
}

fn main() -> Result<()> {
    let program: Result<Vec<usize>, _> = INPUT.split(',').map(str::parse::<usize>).collect();
    let program = program?;

    let part1 = run_program(&program, Some(12), Some(2));
    println!("Part 1: {}", part1);

    for noun in 0..99 {
        for verb in 0..99 {
            let output = run_program(&program, Some(noun), Some(verb));
            if output == 19_690_720 {
                println!("Part 2: {}", noun * 100 + verb);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::run_program;

    #[test]
    fn test_1() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(run_program(&program, None, None), 3500);
    }

    #[test]
    fn test_2() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(run_program(&program, None, None), 30);
    }
}
