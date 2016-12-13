const INPUT: &'static str = include_str!("resources/day_09_input.txt");

struct Marker {
    length: usize,
    repetitions: usize,
}

pub fn problem_1() -> String {
    let mut chars = &mut INPUT.chars();
    let mut out = String::new();
    let mut in_marker = false;
    let mut marker: Option<Marker> = None;
    let mut buf: Vec<char> = vec![];

    loop {
        if let Some(m) = marker {
            let xs: Vec<char> = chars.take(m.length).collect();
            out.extend(xs.into_iter().cycle().take(m.length * m.repetitions));
            marker = None;
        }

        if let Some(c) = chars.next() {
            match c {
                '(' if !in_marker => {
                    in_marker = true;
                    out.extend(buf.drain(0..));
                }
                ')' if in_marker => {
                    in_marker = false;
                    let x: Vec<usize> = buf.drain(0..).collect::<String>()
                        .split('x').map(|x| x.parse().unwrap()).collect();
                    marker = Some(Marker{ length: x[0], repetitions: x[1] });
                }
                '\n' | ' '=> {}
                c => {
                    buf.push(c)
                }
            }
        } else {
            break;
        }
    }
    out.extend(buf.drain(0..));
    out.len().to_string()
}

// pub fn problem_2() -> String {
//     "".into()
// }


mod tests {
    use super::*;

    #[test]
    fn test_problem_1() {
        let answer = problem_1();
        assert_eq!("112830", &answer);
    }

    // #[test]
    // fn test_problem_2() {
    //     let answer = problem_2();
    //     assert_eq!("", &answer);
    // }
}
