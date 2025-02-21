use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // (x, y) => a
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut cp = HashMap::new();
    let mut cnt = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if cp.contains_key(&(i as i64, j as i64)) {
                continue;
            }
            dfs(&grid, i as i64, j as i64, cnt, &mut cp, grid[i][j]);
            cnt += 1;
        }
    }

    let mut v_to_point = HashMap::new();
    for (k, v) in cp {
        v_to_point.entry(v).or_insert(HashSet::new()).insert(k);
    }

    let mut res = 0;
    for (_k, v) in v_to_point {
        // area * permeter
        let v_len = v.len();
        let mut perimeter = 0;
        for (x, y) in v {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0
                    || nx >= grid.len() as i64
                    || ny < 0
                    || ny >= grid[0].len() as i64
                    || grid[nx as usize][ny as usize] != grid[x as usize][y as usize]
                {
                    perimeter += 1;
                }
            }
        }
        res += v_len * perimeter;
    }

    Ok(res.to_string())
}

fn dfs(
    grid: &Vec<Vec<char>>,
    x: i64,
    y: i64,
    cnt: u64,
    cp: &mut HashMap<(i64, i64), u64>,
    current: char,
) {
    if x < 0 || x >= grid.len() as i64 || y < 0 || y >= grid[0].len() as i64 {
        return;
    }

    if cp.contains_key(&(x, y)) {
        return;
    }

    if grid[x as usize][y as usize] != current {
        return;
    }

    cp.insert((x, y), cnt);

    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        dfs(grid, x + dx, y + dy, cnt, cp, current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
