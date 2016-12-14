const INPUT: &'static str = include_str!("resources/day_12_input.txt");

enum Operand {
    Reg(Reg),
    Int(i32),
}

struct Reg(usize);

enum Op {
    Cpy(Operand, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Operand, i32),
}

pub fn problem_1() -> String {
    run([0, 0, 0, 0]).to_string()
}

pub fn problem_2() -> String {
    run([0, 0, 1, 0]).to_string()
}

fn run(mut reg: [i32; 4]) -> i32 {
    let ops: Vec<&str> = INPUT.lines().collect();
    let ops = parse(&ops);
    let mut pc = 0;

    let next_pc = |pc: usize, i: i32| {
        let next = (pc as i32) + (i % ops.len() as i32);
        assert!(next >= 0);
        next as usize
    };

    while pc < ops.len() {
        let mut jump: Option<usize> = None;
        match ops[pc] {
            Op::Cpy(Operand::Reg(Reg(x)), Reg(y)) => reg[y] = reg[x],
            Op::Cpy(Operand::Int(i), Reg(y)) => reg[y] = i,
            Op::Inc(Reg(y)) => reg[y] += 1,
            Op::Dec(Reg(y)) => reg[y] -= 1,
            Op::Jnz(Operand::Reg(Reg(y)), p) if reg[y] != 0 =>
                jump = Some(next_pc(pc, p)),
            Op::Jnz(Operand::Int(i), p) if i != 0 =>
                jump = Some(next_pc(pc, p)),
            Op::Jnz(_, _) => {},
        }
        pc = jump.unwrap_or(pc + 1);
    }
    reg[0]
}

fn parse(raw_ops: &[&str]) -> Vec<Op> {
    raw_ops.iter().map(|line| {
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();
        let mut next = || parts.next().unwrap();
        let opand1 = next();
        match op {
            "cpy" => Op::Cpy(parse_operand(opand1), parse_reg(next())),
            "inc" => Op::Inc(parse_reg(opand1)),
            "dec" => Op::Dec(parse_reg(opand1)),
            "jnz" => Op::Jnz(parse_operand(opand1), next().parse().unwrap()),
            _ => unreachable!()
        }
    })
    .collect()
}

fn parse_reg(s: &str) -> Reg {
    match s {
        "a" => Reg(0),
        "b" => Reg(1),
        "c" => Reg(2),
        "d" => Reg(3),
        _ => unreachable!()
    }
}

fn parse_operand(s: &str) -> Operand {
    match s.parse::<i32>() {
        Ok(i) => Operand::Int(i),
        _ => Operand::Reg(parse_reg(s)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_1() {
        let answer = problem_1();
        assert_eq!("318007", &answer);
    }

    #[test]
    fn test_problem_2() {
        let answer = problem_2();
        assert_eq!("9227661", &answer);
    }
}
