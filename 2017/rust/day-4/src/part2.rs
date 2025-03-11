use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    fn convert(word: &str) -> [u8; 26] {
        let mut res = [0; 26];
        for b in word.bytes() {
            let index = b - b'a';
            res[index as usize] += 1;
        }
        res
    }

    let mut seen = HashSet::new();
    let res = input
        .lines()
        .filter(|line| {
            seen.clear();
            line.split_whitespace().all(|word| {
                let converted = convert(word);
                seen.insert(converted)
            })
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
