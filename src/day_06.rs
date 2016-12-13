use std::collections::HashMap;

const INPUT: &'static str = include_str!("resources/day_06_input.txt");

pub fn problem_1() -> String {
    solve(1)
}

pub fn problem_2() -> String {
    solve(-1)
}

fn solve(sort_sign: i32) -> String {
    let cols = 8;
    let rows = INPUT.lines().count();
    let data: String = INPUT.lines().collect();
    let out: String = (0..cols).map(|col| {
        (0..rows).map(|row| {
            let ix = (row * cols + col) as usize;
            &data[ix..ix+1]
        })
        .fold(HashMap::new(), |mut memo, elem| {
            *memo.entry(elem).or_insert(0) += 1;
            memo
        })
        .into_iter()
        .max_by_key(|&(_c, count)| sort_sign * count)
        .map(|(c, _count)| c)
        .unwrap()
    }).collect();
    out
}


#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn test_problem_1() {
        let t = Instant::now();
        let answer = problem_1();
        println!("Test took: {:?}", t.elapsed());
        assert_eq!("kjxfwkdh", &answer);
    }

    #[test]
    fn test_problem_2() {
        let t = Instant::now();
        let answer = problem_2();
        println!("Test took: {:?}", t.elapsed());
		assert_eq!("xrwcsnps".to_string(), answer);
    }
}
