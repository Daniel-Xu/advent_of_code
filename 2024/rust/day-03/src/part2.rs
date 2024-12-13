use miette::IntoDiagnostic;
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").into_diagnostic()?;
    let mut res = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        res += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    let re = Regex::new(r"don't\(\)([^d]|d[^o])*(?:do\(\))?").into_diagnostic()?;

    for dont_section in re.find_iter(input) {
        let dont_text = dont_section.as_str();

        let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").into_diagnostic()?;
        for (_full, [a, b]) in mul_re.captures_iter(dont_text).map(|c| c.extract()) {
            res -= a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);

        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)unmul(8,5))";
        assert_eq!("8", process(input)?);

        let input = "xmul(1,1)mul[3,7]!^don't()_mul(5,5)+mul(32,64]don't()(mul(11,8)unmul(8,5))";
        assert_eq!("1", process(input)?);

        let input =
            "xmul(1,1)mul[3,7]!^don't()_mul(5,5)+mul(32,64]don't()(mul(11,8)unmul(8,5))domul(1,1)";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
