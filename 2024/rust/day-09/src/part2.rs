use std::collections::{BTreeMap, VecDeque};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let mut empty_m = BTreeMap::new();
    let mut file_dec = VecDeque::new();

    let mut inci = 0;
    for (i, c) in input.chars().enumerate() {
        let file_cnt = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            let file_value = i as i64 / 2;
            file_dec.push_back((file_value, (inci, file_cnt)));
        } else {
            empty_m.insert(inci, file_cnt);
        };
        inci += file_cnt;
    }

    let file_n = file_dec.len();
    for _ in 0..file_n {
        let (file_value, (file_i, file_cnt)) = file_dec.pop_back().unwrap();
        let empty_entry = empty_m
            .iter()
            .find(|(start, cnt)| **start < file_i && **cnt >= file_cnt)
            .map(|(k, _)| *k);

        if let Some(empty_i) = empty_entry {
            let empty_cnt = empty_m[&empty_i];
            let letf = empty_cnt - file_cnt;

            empty_m.remove(&empty_i);
            if letf > 0 {
                empty_m.insert(empty_i + file_cnt, letf);
            }

            file_dec.push_front((file_value, (empty_i, file_cnt)));
        } else {
            file_dec.push_front((file_value, (file_i, file_cnt)));
        }
    }

    let mut res = 0;
    for (file_value, (file_i, file_cnt)) in file_dec {
        for i in 0..file_cnt {
            res += (file_i + i) * file_value as usize;
        }
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
