#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for rows in input.iter() {
        let mut max = 0;
        let mut min = u32::MAX;
        for num in rows.iter() {
            if num > &max {
                max = *num;
            }
            if num < &min {
                min = *num;
            }
        }
        sum += max - min;
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "5 1 9 5
7 5 3
2 4 6 8";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
