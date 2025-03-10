#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let input = input
        .lines()
        .map(|line| {
            let mut line = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            line.sort();
            line
        })
        .collect::<Vec<_>>();

    let mut sum: u32 = 0;
    for rows in input.iter() {
        'outer: for i in 0..rows.len() - 1 {
            for j in i + 1..rows.len() {
                if rows[j] % rows[i] == 0 {
                    sum += rows[j] / rows[i];
                    break 'outer;
                }
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "5 9 2 8
9 4 7 3
3 8 6 5";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
