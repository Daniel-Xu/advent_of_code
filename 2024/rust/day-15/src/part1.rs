#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (grid, instructions) = input.split_once("\n\n").unwrap();
    let mut grid = grid
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '@' {
                start = (i, j);
            }
        }
    }

    let instructions = instructions
        .replace("\n", "")
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect::<Vec<_>>();

    for step in instructions.iter() {
        let (dx, dy) = get_incr(step);

        let (mut nx, mut ny) = (start.0, start.1);
        loop {
            (nx, ny) = ((nx as i64 + dx) as usize, (ny as i64 + dy) as usize);
            match grid[nx][ny] {
                '#' => break,

                '.' => {
                    // replace element backwards
                    if dx != 0 {
                        //vertical
                        let (mut prex, prey) = ((nx as i64 - dx) as usize, ny);

                        while prex != start.0 {
                            let tmp = grid[prex][prey];
                            grid[prex][prey] = grid[nx][ny];
                            grid[nx][ny] = tmp;
                            (nx, ny) = (prex, prey);

                            prex = (prex as i64 - dx) as usize;
                        }
                        grid[prex][prey] = '.';
                        grid[nx][ny] = '@';
                        start.0 = nx;
                        start.1 = ny;
                    } else {
                        // horizontal
                        let (prex, mut prey) = (nx, (ny as i64 - dy) as usize);

                        while prey != start.1 {
                            let tmp = grid[prex][prey];
                            grid[prex][prey] = grid[nx][ny];
                            grid[nx][ny] = tmp;
                            (nx, ny) = (prex, prey);

                            prey = (prey as i64 - dy) as usize;
                        }
                        grid[prex][prey] = '.';
                        grid[nx][ny] = '@';
                        start.0 = nx;
                        start.1 = ny;
                    }
                    break;
                }
                'O' => continue,

                _ => unreachable!(),
            }
        }

        // println!(
        //     "{}",
        //     grid.iter()
        //         .map(|row| row.iter().collect::<String>())
        //         .collect::<Vec<_>>()
        //         .join("\n")
        // );
    }

    let mut res = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'O' {
                res += i * 100 + j;
            }
        }
    }

    Ok(res.to_string())
}

fn get_incr(step: &Direction) -> (i64, i64) {
    match step {
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
        Direction::Up => (-1, 0),
    }
}
#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = " ########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!("2028", process(input)?);
        Ok(())
    }
}
