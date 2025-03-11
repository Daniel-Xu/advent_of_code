use std::cmp::min;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let num: i64 = input.parse::<i64>().unwrap();
    let mut current_layer = 1;
    let mut first_number = 1;
    let mut current_max = 1;
    loop {
        if num <= current_max {
            break;
        }

        current_layer += 1;
        first_number = current_max + 1;
        current_max = first_number + (current_layer * 2 - 1) * 4 - 4 - 1;
    }

    let side_length = current_layer * 2 - 1;
    let right_mid = first_number + side_length / 2 - 1;
    let top_mid = right_mid + side_length - 1;
    let left_mid = top_mid + side_length - 1;
    let bottom_mid = left_mid + side_length - 1;
    let distance = min(
        min((num - right_mid).abs(), (num - left_mid).abs()),
        min((num - top_mid).abs(), (num - bottom_mid).abs()),
    ) + current_layer
        - 1;

    Ok(format!("{}", distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1024";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
