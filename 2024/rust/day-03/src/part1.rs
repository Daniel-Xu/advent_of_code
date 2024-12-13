use miette::IntoDiagnostic;
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").into_diagnostic()?;
    let mut res = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        res += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
