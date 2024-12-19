use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut antennas = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            // lowercase letter, uppercase letter, or digit
            if cell.is_ascii_alphabetic() || cell.is_ascii_digit() {
                antennas.entry(cell).or_insert(vec![]).push((i, j));
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    let m = grid.len();
    let n = grid[0].len();

    for (_point, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = (positions[i].0 as i64, positions[i].1 as i64);
                let (x2, y2) = (positions[j].0 as i64, positions[j].1 as i64);
                let x_len = (x2 - x1).abs();
                let y_len = (y2 - y1).abs();

                let (x3, y3, x4, y4) = match (x1 < x2, y1 < y2) {
                    (true, true) => (x1 - x_len, y1 - y_len, x2 + x_len, y2 + y_len),
                    (true, false) => (x1 - x_len, y1 + y_len, x2 + x_len, y2 - y_len),
                    (false, true) => (x1 + x_len, y1 - y_len, x2 - x_len, y2 + y_len),
                    (false, false) => (x1 + x_len, y1 + y_len, x2 - x_len, y2 - y_len),
                };

                if x3 >= 0 && x3 < m as i64 && y3 >= 0 && y3 < n as i64 {
                    antinodes.insert((x3 as usize, y3 as usize));
                }

                if x4 >= 0 && x4 < m as i64 && y4 >= 0 && y4 < n as i64 {
                    antinodes.insert((x4 as usize, y4 as usize));
                }
            }
        }
    }

    Ok(antinodes.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
