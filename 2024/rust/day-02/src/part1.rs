#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut res = 0;

    for line in input.lines() {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut increasing = true;
        for i in 1..numbers.len() {
            if numbers[i] == numbers[i - 1] {
                break;
            }

            if i == 1 {
                increasing = numbers[i] > numbers[i - 1];
            }

            if increasing {
                if numbers[i] < numbers[i - 1] {
                    break;
                }
            } else if numbers[i] > numbers[i - 1] {
                break;
            }

            let diff = (numbers[i] - numbers[i - 1]).abs();
            if !(1..=3).contains(&diff) {
                break;
            }

            if i == numbers.len() - 1 {
                res += 1;
            }
        }
    }

    Ok(format!("{}", res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
        7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
