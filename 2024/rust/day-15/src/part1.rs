#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!("1", process(input)?);
        Ok(())
    }
}
