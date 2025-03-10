#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let input = input.chars().collect::<Vec<_>>();

    let mut sum: u64 = 0;
    let half_len = input.len() / 2;
    for i in 0..input.len() {
        let j = (i + half_len) % input.len();

        if input[i] == input[j] {
            sum += input[i].to_digit(10).unwrap() as u64;
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let test_cases = vec![
            ("1212", "6"),
            ("1221", "0"),
            ("123425", "4"),
            ("123123", "12"),
        ];

        for (input, expected) in test_cases {
            println!("\nTesting input: {}", input);
            assert_eq!(expected, process(input)?);
        }
        Ok(())
    }
}
