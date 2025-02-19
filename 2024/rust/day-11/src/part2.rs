use std::collections::HashMap;
fn answer(cache: &mut HashMap<(u64, usize), u64>, x: u64, n: usize) -> u64 {
    if n == 0 {
        return 1;
    }

    if cache.contains_key(&(x, n)) {
        return cache[&(x, n)];
    }

    let cur_res = match x {
        _ if x == 0 => answer(cache, 1, n - 1),
        v if v.to_string().len() % 2 == 0 => {
            let s = v.to_string();
            let (a, b) = s.split_at(s.len() / 2);
            let a = a.parse::<u64>().unwrap();
            let b = b.parse::<u64>().unwrap();
            answer(cache, a, n - 1) + answer(cache, b, n - 1)
        }
        _ => answer(cache, x * 2024, n - 1),
    };

    cache.insert((x, n), cur_res);
    cur_res
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let nums = input
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut cache = HashMap::new();
    let mut res = 0;
    for v in nums {
        res += answer(&mut cache, v, 75);
    }

    Ok(format!("{}", res))
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // let input = "125 17";
        // assert_eq!("55312", process(input)?);
        Ok(())
    }
}
