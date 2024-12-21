#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 0u32;
    for (i, row) in grid.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            if val == 0 {
                let mut path = vec![0];
                let temp = traverse(&grid, &mut path, i as i32, j as i32);
                res += temp;
            }
        }
    }

    Ok(res.to_string())
}

fn traverse(grid: &Vec<Vec<u32>>, path: &mut Vec<u32>, i: i32, j: i32) -> u32 {
    let m = grid.len();
    let n = grid[0].len();

    if path.len() == 10 {
        return 1;
    }

    let mut res = 0;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let x = i + dx;
        let y = j + dy;
        if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
            continue;
        }

        if grid[x as usize][y as usize] == path[path.len() - 1] + 1 {
            path.push(grid[x as usize][y as usize]);
            res += traverse(grid, path, x, y);
            path.pop();
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        //         let input = "0123
        // 1234
        // 8765
        // 9876";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
