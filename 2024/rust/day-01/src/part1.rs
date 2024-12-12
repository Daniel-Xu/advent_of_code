#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines = input.lines();
    let mut first_col = vec![];
    let mut second_col = vec![];

    for line in lines {
        let mut locations = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        first_col.push(locations.next().unwrap());
        second_col.push(locations.next().unwrap());
    }

    first_col.sort();
    second_col.sort();

    let res = std::iter::zip(first_col, second_col)
        .map(|(first, second)| (first - second).abs())
        .sum::<i32>();

    Ok(format!("{}", res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
