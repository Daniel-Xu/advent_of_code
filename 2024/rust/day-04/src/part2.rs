#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut res = 0;
    let m = grid.len();
    let n = grid[0].len();

    for i in 0..=m - 3 {
        for j in 0..=n - 3 {
            let mut first = String::new();
            let mut second = String::new();
            for k in 0..3 {
                first.push(grid[i + k][j + k]);
                second.push(grid[i + 2 - k][j + k]);
            }
            if (first == "MAS" || first == "SAM") && (second == "MAS" || second == "SAM") {
                res += 1;
            }
        }
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
