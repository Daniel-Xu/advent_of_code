use std::collections::VecDeque;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut nums = input
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<VecDeque<u64>>();

    for _ in 0..25 {
        let n = nums.len();

        for _ in 0..n {
            let current = nums.pop_front().unwrap();

            if current == 0 {
                // replace with 1
                nums.push_back(1);
            } else if current.to_string().len() % 2 == 0 {
                // split into two halves
                let s = current.to_string();
                let (a, b) = s.split_at(s.len() / 2);
                nums.push_back(a.parse::<u64>().unwrap());
                nums.push_back(b.parse::<u64>().unwrap());
            } else {
                // * 2024
                nums.push_back(current * 2024);
            }
        }
    }
    Ok(format!("{}", nums.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
