#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let input = input.chars().collect::<Vec<_>>();

    let mut sum: u64 = 0;
    for i in 0..input.len() {
        let mut j = i + 1;
        if j >= input.len() {
            j = 0;
        }

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
            ("1122", "3"),
            ("1111", "4"),
            ("1234", "0"),
            ("91212129", "9"),
        ];

        for (input, expected) in test_cases {
            println!("\nTesting input: {}", input);
            assert_eq!(expected, process(input)?);
        }
        Ok(())
    }
}
