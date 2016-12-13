use std::fmt::Write;
use std::collections::HashMap;

use md5;

const INPUT: &'static str = "uqwqemis";
const PAT: &'static str = "00000";

pub fn problem_1() -> String {
    let out = (0_usize..)
        .take(100000000)
        .map(|i| hex(&compute(i)[0..3]))
        .filter(|s| &s[0..5] == PAT)
        .map(|s| s.chars().nth(5).unwrap())
        .take(8)
        .collect();
    out
}

pub fn problem_2() -> String {
    let mut aggregator = HashMap::new();
    (0_usize..)
        .take(100000000)
        .map(|i| hex(&compute(i)[0..4]))
        .filter(|s| {
            let pos = s[5..6].chars().next().unwrap();
            &s[0..5] == PAT &&
                pos.to_digit(10).map(|d| d <= 7).unwrap_or(false)
        })
        .map(|s|
             (s.chars().nth(5).unwrap().to_digit(10).unwrap(),
              s.chars().nth(6).unwrap()))
        .scan(&mut aggregator, |memo, (pos, c)| {
            if memo.contains_key(&pos) {
                Some(())
            } else {
                memo.insert(pos, c);
                if memo.keys().count() == 8 {
                    None
                } else {
                    Some(())
                }
            }
        })
        .last();
    let mut x: Vec<_> = aggregator.iter().collect();
    x.sort_by_key(|&(k, _v)| k);
    x.into_iter().map(|(_k, v)| *v).collect()
}

fn compute<'a>(i: usize) -> [u8; 16] {
    md5::compute(format!("{}{}", INPUT, i).as_bytes())
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}


#[cfg(test)]
mod tests {
    // use std::time::Instant;
    // use super::*;
    use super::hex;

    // #[test]
    // fn test_problem_1() {
    //     let t = Instant::now();
    //     let answer = problem_1();
    //     println!("Test took: {:?}", t.elapsed());
    //     assert_eq!("1a3099aa", &answer);
    // }

    #[test]
    fn test_hex() {
        assert_eq!("00000d".to_string(), hex("\0\0\r".as_bytes()));
    }

    // #[test]
    // fn test_problem_2() {
    //     let t = Instant::now();
    //     let answer = problem_2();
    //     println!("Test took: {:?}", t.elapsed());
		// assert_eq!("694190cd".to_string(), answer);
    // }
}
