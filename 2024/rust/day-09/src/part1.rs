#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let n = input
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0) as u64)
        .sum::<u64>();

    let mut v: Vec<i64> = vec![-1; n as usize];
    let mut i = 0;
    for (j, c) in input.chars().enumerate() {
        let mut value = j as i64 / 2;
        if j % 2 != 0 {
            value = -1;
        }

        let cnt = c.to_digit(10).unwrap_or(0) as usize;
        v[i..i + cnt].fill(value);
        i += cnt;
    }
    let (mut i, mut j) = (0usize, (n - 1) as usize);
    while i < j {
        if v[i] != -1 {
            i += 1;
            continue;
        }
        if v[j] == -1 {
            j -= 1;
            continue;
        }
        v.swap(i, j);
        i += 1;
        j -= 1;
    }
    let res = v
        .iter()
        .enumerate()
        .filter(|(_, &val)| val != -1)
        .map(|(i, &val)| val as u64 * i as u64)
        .sum::<u64>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        // let input = "12345";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
