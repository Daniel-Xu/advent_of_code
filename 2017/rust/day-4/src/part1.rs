use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let res = input
        .lines()
        .filter(|line| {
            let words = line.split_whitespace();
            let word_set = HashSet::<&str>::from_iter(words.clone());
            word_set.len() == words.count()
        })
        .count();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa
";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
