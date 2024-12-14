use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (m, n) = (grid.len(), grid[0].len());
    let (mut startx, mut starty) = (0, 0);

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                startx = i;
                starty = j;
                break;
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert((startx, starty));

    let mut diri = 0;
    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut x, mut y) = (startx as i32, starty as i32);
    let m = m as i32;
    let n = n as i32;

    while x >= 0 && x < m && y >= 0 && y < n {
        let (dx, dy) = dir[diri];
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || nx >= m || ny < 0 || ny >= n {
            break;
        }
        if grid[nx as usize][ny as usize] == '#' {
            diri += 1;
            diri %= 4;
        } else {
            visited.insert((nx as usize, ny as usize));
            x = nx;
            y = ny;
        }
    }

    Ok(format!("{}", visited.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
