use std::collections::HashMap;

const INPUT: &'static str = include_str!("resources/day_04_input.txt");

struct Entry {
    encrypted: Vec<String>,
    sector_id: u32,
}

pub fn problem_1() -> String {
    let sector_sum: u32 = extract().iter().map(|e|e.sector_id).sum();
    sector_sum.to_string()
}

pub fn problem_2() -> (String, u32) {
    let decrypted: Vec<_> = extract().iter().map(|entry| {
		let decrypted = entry.encrypted.iter().map(|token| {
            token.bytes().map(|c| {
				let len: u32 = (b'z' - b'a' + 1) as u32;
				let normalized: u32 = (c - b'a') as u32;
				let next = ((entry.sector_id + normalized) % len) + b'a' as u32;
				char::from(next as u8)
            }).collect::<String>()
        }).collect::<Vec<_>>().join(" ");
		(decrypted, entry.sector_id)
    }).collect();
	decrypted.into_iter().filter(|pair| pair.0.contains("pole"))
		.next().unwrap()
}

fn extract() -> Vec<Entry> {
    INPUT.lines().flat_map(|line| {
        let tokens: Vec<String> = line.split("-").map(|x|x.to_owned()).collect();
        let encrypted_name: Vec<String> = tokens.iter().take(tokens.len() -1)
            .cloned().collect();
        let name_tokens: Vec<char> =
            encrypted_name.iter().flat_map(|s| s.chars()).collect();
        let mut counted: Vec<(char, i32)> =
            name_tokens.iter().fold(HashMap::new(), |mut memo, elem| {
                *memo.entry(*elem).or_insert(0) += 1;
                memo
            }).into_iter().collect();
        counted.sort_by_key(|&(c, count)| (-count, c));

        let (sector_id, checksum) = {
            let mut itr = tokens.last().unwrap().split("[")
                .map(|x| x.replace("]", ""));
            (itr.next().unwrap(), itr.next().unwrap())
        };
        let calculated_checksum: String =
            counted.iter().map(|&(c, _count)| c).take(5).collect();
        if calculated_checksum == checksum {
            Some( Entry {
                encrypted: encrypted_name,
                sector_id: sector_id.parse::<u32>().unwrap()
            })
        } else {
            None
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_1() {
        let answer = problem_1();
        assert_eq!("361724", &answer);
    }

    #[test]
    fn test_problem_2() {
        let answer = problem_2();
		assert_eq!(("northpole object storage".into(), 482), answer);
    }
}
