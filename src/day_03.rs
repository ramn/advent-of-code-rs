const INPUT: &'static str = include_str!("resources/day_03_input.txt");

pub fn problem_1() -> String {
    let input = INPUT.lines().map(|line| {
        let numbers: Vec<u32> = line.split_whitespace()
            .map(|token| token.parse().unwrap()).collect();
        numbers
    }).collect();
    solve(input)
}

pub fn problem_2() -> String {
    let transposed: Vec<u32> = INPUT.lines()
        .map(|line| line.split_whitespace())
        .fold(vec![vec![], vec![], vec![]], |mut memo, elem| {
            for (ix, number) in elem.enumerate() {
                memo[ix].push(number.parse::<u32>().unwrap())
            }
            memo
        })
        .into_iter()
        .fold(vec![], |mut memo, elem| {
            memo.extend(elem);
            memo
        });
        solve(transposed.chunks(3).map(|slice| Vec::from(slice)).collect())
}

fn solve(input: Vec<Vec<u32>>) -> String {
    let answer = input.into_iter().map(|numbers| {
        let valid_triangle = numbers.iter()
            .zip(numbers.iter().cycle().skip(1))
            .zip(numbers.iter().cycle().skip(2))
            .all(|((a, b), &c)| {
                a + b > c
            });
        valid_triangle
    }).filter(|&x| x).count();
    answer.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_1() {
        let answer = problem_1();
        assert_eq!("993", &answer);
    }

    #[test]
    fn test_problem_2() {
        let answer = problem_2();
        assert_eq!("1849", &answer);
    }
}
