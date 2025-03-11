use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let res = input
        .lines()
        .filter(|line| {
            let words = line.split_whitespace().map(|word| {
                let mut chars = word.chars().collect::<Vec<char>>();
                chars.sort_unstable();
                chars.iter().collect::<String>()
            });
            let word_set = HashSet::<String>::from_iter(words.clone());
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
        let input = "abcde xyz ecdab
abcde fghij
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio
";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
