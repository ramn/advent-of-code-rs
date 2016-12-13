use std::ops::Add;
use std::ops::AddAssign;
use std::cmp::{min, max};


pub fn problem_1() -> String {
    let start = Pad1 { v: Vec2 { x: 0, y: 0 }};
    solve(start)
}

pub fn problem_2() -> String {
    let start = Pad2 { v: Vec2 { x: 0, y: 0 }};
    solve(start)
}

fn solve<T: Pad>(start: T) -> String {
    let answer: String = INPUT.lines()
        .fold((String::new(), start), |(mut keys, pad), line| {
            let next_pad = line.chars()
                .map(turn_to_vec)
                .fold(pad, |mut pad, elem| { pad.step(elem); pad });
            keys.extend(next_pad.get_data().chars());
            (keys, next_pad)
        }).0;
    answer
}

fn turn_to_vec(c: char) -> Vec2 {
    match c {
        'R' => Vec2 { x: 1, y: 0 },
        'L' => Vec2 { x: -1, y: 0 },
        'U' => Vec2 { x: 0, y: 1 },
        'D' => Vec2 { x: 0, y: -1 },
        unexpected => {
            println!("Found unexpected char: '{}'", unexpected);
            unreachable!()
        }
    }
}

trait Pad {
    fn step(&mut self, v: Vec2);
    fn get_vec(&self) -> Vec2;
    fn get_data<'a>(&'a self) -> &'a str;
    fn get_ix(&self) -> usize;
}

struct Pad1 {
    v: Vec2,
}

impl Pad1 {
    fn normalize(&self, v: Vec2) -> Vec2 {
        Vec2 {
            x: max(-1, min(1, v.x)),
            y: max(-1, min(1, v.y)),
        }
    }
}

impl Pad for Pad1 {
    fn step(&mut self, v: Vec2) {
        self.v = self.normalize(self.v + v);
    }

    fn get_vec(&self) -> Vec2 {
        self.v
    }

    fn get_data<'a>(&'a self) -> &'a str {
        let ix = self.get_ix();
        let pad = "789456123";
        &pad[ix..ix+1]
    }

    fn get_ix(&self) -> usize {
        let rows = 3;
        let cols = 3;
        (self.v.y * rows + self.v.x + (rows * cols / 2)) as usize
    }
}

#[derive(Debug)]
struct Pad2 {
    v: Vec2,
}

impl Pad2 {
    fn can_take_step(&self, v: Vec2) -> bool {
        let next = self.v + v;
        (next.x.abs() + next.y.abs()) <= 2
    }
}

impl Pad for Pad2 {
    fn step(&mut self, v: Vec2) {
        if self.can_take_step(v) {
            self.v += v;
        }
    }

    fn get_vec(&self) -> Vec2 {
        self.v
    }

    fn get_data<'a>(&'a self) -> &'a str {
        let ix = self.get_ix();
        let pad = "  D   ABC 56789 234   1  ";
        &pad[ix..ix + 1]
    }

    fn get_ix(&self) -> usize {
        let rows = 5;
        let cols = 5;
        (self.v.y * rows + self.v.x + (rows * cols / 2)) as usize
    }
}


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec2 {x: self.x + other.x, y: self.y + other.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other
    }
}


const INPUT: &'static str =
"RDLULDLDDRLLLRLRULDRLDDRRRRURLRLDLULDLDLDRULDDLLDRDRUDLLDDRDULLLULLDULRRLDURULDRUULLLUUDURURRDDLDLDRRDDLRURLLDRRRDULDRULURURURURLLRRLUDULDRULLDURRRLLDURDRRUUURDRLLDRURULRUDULRRRRRDLRLLDRRRDLDUUDDDUDLDRUURRLLUDUDDRRLRRDRUUDUUULDUUDLRDLDLLDLLLLRRURDLDUURRLLDLDLLRLLRULDDRLDLUDLDDLRDRRDLULRLLLRUDDURLDLLULRDUUDRRLDUDUDLUURDURRDDLLDRRRLUDULDULDDLLULDDDRRLLDURURURUUURRURRUUDUUURULDLRULRURDLDRDDULDDULLURDDUDDRDRRULRUURRDDRLLUURDRDDRUDLUUDURRRLLRR
RDRRLURDDDDLDUDLDRURRLDLLLDDLURLLRULLULUUURLDURURULDLURRLRULDDUULULLLRLLRDRRUUDLUUDDUDDDRDURLUDDRULRULDDDLULRDDURRUURLRRLRULLURRDURRRURLDULULURULRRLRLUURRRUDDLURRDDUUDRDLLDRLRURUDLDLLLLDLRURDLLRDDUDDLDLDRRDLRDRDLRRRRUDUUDDRDLULUDLUURLDUDRRRRRLUUUDRRDLULLRRLRLDDDLLDLLRDDUUUUDDULUDDDUULDDUUDURRDLURLLRUUUUDUDRLDDDURDRLDRLRDRULRRDDDRDRRRLRDULUUULDLDDDUURRURLDLDLLDLUDDLDLRUDRLRLDURUDDURLDRDDLLDDLDRURRULLURULUUUUDLRLUUUDLDRUDURLRULLRLLUUULURLLLDULLUDLLRULRRLURRRRLRDRRLLULLLDURDLLDLUDLDUDURLURDLUURRRLRLLDRLDLDRLRUUUDRLRUDUUUR
LLLLULRDUUDUUDRDUUURDLLRRLUDDDRLDUUDDURLDUDULDRRRDDLLLRDDUDDLLLRRLURDULRUUDDRRDLRLRUUULDDULDUUUDDLLDDDDDURLDRLDDDDRRDURRDRRRUUDUUDRLRRRUURUDURLRLDURDDDUDDUDDDUUDRUDULDDRDLULRURDUUDLRRDDRRDLRDLRDLULRLLRLRLDLRULDDDDRLDUURLUUDLLRRLLLUUULURUUDULRRRULURUURLDLLRURUUDUDLLUDLDRLLRRUUDDRLUDUDRDDRRDDDURDRUDLLDLUUDRURDLLULLLLUDLRRRUULLRRDDUDDDUDDRDRRULURRUUDLUDLDRLLLLDLUULLULLDDUDLULRDRLDRDLUDUDRRRRLRDLLLDURLULUDDRURRDRUDLLDRURRUUDDDRDUUULDURRULDLLDLDLRDUDURRRRDLDRRLUDURLUDRRLUDDLLDUULLDURRLRDRLURURLUUURRLUDRRLLULUULUDRUDRDLUL
LRUULRRUDUDDLRRDURRUURDURURLULRDUUDUDLDRRULURUDURURDRLDDLRUURLLRDLURRULRRRUDULRRULDLUULDULLULLDUDLLUUULDLRDRRLUURURLLUUUDDLLURDUDURULRDLDUULDDRULLUUUURDDRUURDDDRUUUDRUULDLLULDLURLRRLRULRLDLDURLRLDLRRRUURLUUDULLLRRURRRLRULLRLUUDULDULRDDRDRRURDDRRLULRDURDDDDDLLRRDLLUUURUULUDLLDDULDUDUUDDRURDDURDDRLURUDRDRRULLLURLUULRLUDUDDUUULDRRRRDLRLDLLDRRDUDUUURLRURDDDRURRUDRUURUUDLRDDDLUDLRUURULRRLDDULRULDRLRLLDRLURRUUDRRRLRDDRLDDLLURLLUDL
ULURLRDLRUDLLDUDDRUUULULUDDDDDRRDRULUDRRUDLRRRLUDLRUULRDDRRLRUDLUDULRULLUURLLRLLLLDRDUURDUUULLRULUUUDRDRDRUULURDULDLRRULUURURDULULDRRURDLRUDLULULULUDLLUURULDLLLRDUDDRRLULUDDRLLLRURDDLDLRLLLRDLDRRUUULRLRDDDDRUDRUULDDRRULLDRRLDDRRUDRLLDUDRRUDDRDLRUDDRDDDRLLRDUULRDRLDUDRLDDLLDDDUUDDRULLDLLDRDRRUDDUUURLLUURDLULUDRUUUDURURLRRDULLDRDDRLRDULRDRURRUDLDDRRRLUDRLRRRRLLDDLLRLDUDUDDRRRUULDRURDLLDLUULDLDLDUUDDULUDUDRRDRLDRDURDUULDURDRRDRRLLRLDLU";


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_1() {
        let answer = problem_1();
        assert_eq!("33444", &answer);
    }

    #[test]
    fn test_problem_2() {
        let answer = problem_2();
        assert_eq!("446A6", &answer);
    }
}
