use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines = input.lines();
    let mut first_col = vec![];
    let mut second_col = HashMap::new();

    for line in lines {
        let mut locations = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        first_col.push(locations.next().unwrap());
        second_col
            .entry(locations.next().unwrap())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    first_col.sort();

    let res = first_col.into_iter().fold(0, |acc, first| {
        acc + first * second_col.get(&first).unwrap_or(&0)
    });

    Ok(format!("{}", res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3 ";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
