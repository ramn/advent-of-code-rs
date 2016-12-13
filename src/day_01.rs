use std::collections::HashSet;


const INPUT: &'static str = "R4, R5, L5, L5, L3, R2, R1, R1, L5, R5, R2, \
    L1, L3, L4, R3, L1, L1, R2, R3, R3, R1, L3, L5, R3, R1, L1, R1, R2, L1, \
    L4, L5, R4, R2, L192, R5, L2, R53, R1, L5, R73, R5, L5, R186, L3, L2, \
    R1, R3, L3, L3, R1, L4, L2, R3, L5, R4, R3, R1, L1, R5, R2, R1, R1, R1, \
    R3, R2, L1, R5, R1, L5, R2, L2, L4, R3, L1, R4, L5, R4, R3, L5, L3, R4, \
    R2, L5, L5, R2, R3, R5, R4, R2, R1, L1, L5, L2, L3, L4, L5, L4, L5, L1, \
    R3, R4, R5, R3, L5, L4, L3, L1, L4, R2, R5, R5, R4, L2, L4, R3, R1, L2, \
    R5, L5, R1, R1, L1, L5, L5, L2, L1, R5, R2, L4, L1, R4, R3, L3, R1, R5, \
    L1, L4, R2, L3, R5, R3, R1, L3";

pub fn day_1_problem_1() -> i32 {
    let folded = INPUT.split(",").map(|s| s.trim())
        .fold((0, [0i32; 4]), |(prev_dir, mut cardinal_steps), elem| {
            let turn = match &elem[0..1] {
                "R" => 1,
                "L" => 3,
                _ => unreachable!(),
            };
            let steps: i32 = elem[1..].parse().unwrap();
            let dir = (prev_dir + turn) % 4;
            cardinal_steps[dir] += steps;
            (dir, cardinal_steps)
        });
    let steps = folded.1;
    let manhattan_distance =
        (steps[0] - steps[2]).abs() + (steps[1] - steps[3]).abs();
    manhattan_distance
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Coord {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Memo {
    coords: HashSet<Coord>,
    last_coord: Coord,
    last_dir: u32,
    done: bool,
}

impl Memo {
    fn new() -> Memo {
        Memo {
            coords: HashSet::new(),
            last_coord: Coord { row: 0, col: 0 },
            last_dir: 0,
            done: false,
        }
    }
}

pub fn day_1_problem_2() -> i32 {
    let goal_coord = INPUT.split(",").map(|s| s.trim())
        .scan(Memo::new(), step)
        .last().unwrap();
    goal_coord.row.abs() + goal_coord.col.abs()
}

fn step(memo: &mut Memo, elem: &str) -> Option<Coord> {
    let dir = calc_dir(memo.last_dir, &elem[0..1]);
    let coord = {
        let steps: i32 = elem[1..].parse().unwrap();
        let last = &memo.last_coord;
        match dir {
            0 => Coord { row: last.row - steps, col: last.col },
            1 => Coord { row: last.row, col: last.col + steps },
            2 => Coord { row: last.row + steps, col: last.col },
            3 => Coord { row: last.row, col: last.col - steps },
            _ => unreachable!(),
        }
    };
    let coords_last_section = coords_for_section(&memo.last_coord, &coord);
    let crossing = coords_last_section.iter()
        .skip(1).find(|coord| memo.coords.contains(coord)).cloned();
    if memo.done {
        None
    } else if let Some(goal) = crossing {
        memo.done = true;
        Some(goal)
    } else {
        memo.coords.extend(coords_last_section.into_iter());
        memo.last_coord = coord.clone();
        memo.last_dir = dir;
        Some(coord.clone())
    }
}

fn range(a: i32, b: i32) -> Vec<i32> {
    if a < b {
        (a..b+1).collect()
    } else {
        (b..a+1).rev().collect()
    }
}

fn coords_for_section(last: &Coord, this: &Coord) -> Vec<Coord> {
    if last.row == this.row {
        range(last.col, this.col).into_iter()
            .map(|col| Coord { row: this.row, col: col })
            .collect()
    } else if last.col == this.col {
        range(last.row, this.row).into_iter()
            .map(|row| Coord { row: row, col: this.col })
            .collect()
    } else {
        panic!("Coords are not on a straight line.")
    }
}

fn calc_dir(last_dir: u32, turn: &str) -> u32 {
    let delta = match turn {
        "R" => 1,
        "L" => 3,
        _ => unreachable!(),
    };
    (last_dir + delta) % 4
}


#[cfg(test)]
mod tests {
    use super::day_1_problem_1;
    use super::day_1_problem_2;

    #[test]
    fn tes_day_1_problem_1() {
        let answer = day_1_problem_1();
        assert_eq!(answer, 250);
        // println!("Answer: {}", answer);
    }

    #[test]
    fn tes_day_1_problem_2() {
        let answer = day_1_problem_2();
        assert_eq!(answer, 151);
        // println!("Answer: {}", answer);
    }
}
