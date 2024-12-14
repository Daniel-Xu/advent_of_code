#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;

    for x in 0..grid.len() as i32 {
        for y in 0..grid[0].len() as i32 {
            for (dx, dy) in [
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                let mut temp = String::new();
                for m in 0..4 {
                    let (nx, ny) = (x + dx * m, y + dy * m);
                    if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                        break;
                    }
                    temp.push(grid[nx as usize][ny as usize]);
                }
                if temp == "XMAS" {
                    res += 1;
                }
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
        let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
