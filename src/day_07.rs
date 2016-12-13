use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("resources/day_07_input.txt");

#[derive(Debug)]
enum Block {
    Hypernet(Vec<char>),
    Supernet(Vec<char>),
}

#[derive(Eq, PartialEq)]
enum Fits {
    Undecided,
    Support,
    NoSupport,
}

pub fn problem_1() -> String {
    let out = extract().into_iter().map(|line| {
        line.into_iter().fold(Fits::Undecided, |memo, elem| {
            let next = match elem {
                Block::Hypernet(ref block) if has_abba(block) =>
                    Fits::NoSupport,
                Block::Supernet(ref block) if has_abba(block) =>
                    Fits::Support,
                _ =>
                    Fits::Undecided
            };
            match memo {
                Fits::NoSupport => Fits::NoSupport,
                Fits::Undecided => next,
                Fits::Support if next == Fits::NoSupport => Fits::NoSupport,
                Fits::Support => Fits::Support,
            }
        })
    })
    .filter(|x| x == &Fits::Support)
    .count();
    out.to_string()
}

#[derive(Hash, Eq, PartialEq)]
enum SslBlock {
    ABA,
    BAB,
}
pub fn problem_2() -> String {
    let out = extract().into_iter().map(|line| {
        let mut matching_blocks =
            line.into_iter().fold(HashMap::new(), |mut memo, part| {
                let (key, matches) = match part {
                    Block::Supernet(block) =>
                        (SslBlock::ABA, get_ababs(&block[..])),
                    Block::Hypernet(block) =>
                        (SslBlock::BAB, flip_all(get_ababs(&block[..]))),
                };
                memo.entry(key).or_insert(HashSet::new())
                    .extend(matches);
                memo
            });
        let abas = matching_blocks.remove(&SslBlock::ABA);
        let babs = matching_blocks.remove(&SslBlock::BAB);
        abas.and_then(|abas|
            babs.map(|babs| abas.intersection(&babs).count() > 0
        )).unwrap_or(false)
    })
    .filter(|&x| x)
    .count();
    out.to_string()
}

fn get_ababs(block: &[char]) -> Vec<String> {
    block.windows(3)
        .filter(|w| w[0..1] == w[2..3] && w[0..1] != w[1..2])
        .map(|chars| chars.into_iter().cloned().collect())
        .collect()
}

fn flip_all(v: Vec<String>) -> Vec<String> {
    v.into_iter().map(flip).collect()
}

fn flip(v: String) -> String {
    let mut s = String::new();
    s.push_str(&v[1..2]);
    s.push_str(&v[0..1]);
    s.push_str(&v[1..2]);
    s
}

fn has_abba(block: &[char]) -> bool {
    block.windows(4)
        .map(|w| w[0..1] == w[3..4] && w[1..2] == w[2..3] && w[0..1] != w[1..2])
        .any(|x: bool| x)
}

fn extract() -> Vec<Vec<Block>> {
    let out = INPUT.lines().map(|line| {
        let parts = {
            let (last_elem, mut memo) =
                line.chars().fold((vec![], vec![]), |mut memo, elem| {
                    match elem {
                        '[' | ']' => {
                            if !memo.0.is_empty() {
                                memo.1.push(memo.0.split_off(0));
                            }
                        }
                        c => {
                            memo.0.push(c);
                        }
                    };
                    memo
                });
            memo.push(last_elem);
            memo
        };
        let is_hypernet_init = &line[0..1] == "[";
        let init = (is_hypernet_init, vec![]);
        parts.into_iter().fold(init, |mut memo, elem: Vec<char>| {
            let block =
                if memo.0 { Block::Hypernet(elem) }
                else { Block::Supernet(elem) };
            memo.1.push(block);
            (!memo.0, memo.1)
        }).1
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
        assert_eq!("118", &answer);
    }

    #[test]
    fn test_problem_2() {
        let t = Instant::now();
        let answer = problem_2();
        println!("Test took: {:?}", t.elapsed());
		assert_eq!("260".to_string(), answer);
    }

    #[test]
    fn test_splitting_with_slice_pattern() {
        let s: Vec<String> = "aaa[bbb]ccc"
            .split(['[', ']'].as_ref())
            .map(|x| x.to_string()).collect();
        assert_eq!("aaa", s[0]);
        assert_eq!("bbb", s[1]);
        assert_eq!("ccc", s[2]);
    }
}
