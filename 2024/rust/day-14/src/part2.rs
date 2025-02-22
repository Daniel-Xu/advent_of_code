#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    const ROW: i32 = 103;
    const COL: i32 = 101;

    let original_robots = Robot::parse(input);
    let total_robots = original_robots.len();
    println!("total robot {}", total_robots);

    // for t in 0..100000 {
    let t = 6577;
    let mut robots = original_robots.clone();
    let mut visited = HashSet::new();

    for robot in robots.iter_mut() {
        let (x, y) = robot.position;
        let (vx, vy) = robot.velocity;

        robot.position = ((x + vx * t).rem_euclid(COL), (y + vy * t).rem_euclid(ROW));
        visited.insert(robot.position);
    }

    // if visited.len() == total_robots {
    //     println!("len: {}, t: {}", visited.len(), t);
    //     break;
    // }

    let mut grid = vec![vec!["."; COL as usize]; ROW as usize];

    for robot in &robots {
        let (x, y) = robot.position;
        grid[y as usize][x as usize] = "#";
    }

    println!(
        "{}",
        grid.iter()
            .map(|row| row.join(""))
            .collect::<Vec<_>>()
            .join("\n")
    );
    // }

    Ok(0.to_string())
}
#[derive(Debug, Clone, Copy)]
struct Robot {
    // x is col, y is row
    position: (i32, i32),
    velocity: (i32, i32),
}

use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};

impl Robot {
    fn parse(input: &str) -> Vec<Robot> {
        input
            .lines()
            .map(|line| {
                let (_, robot) = Self::parse_line(line).unwrap();
                robot
            })
            .collect()
    }

    fn parse_line(input: &str) -> IResult<&str, Robot> {
        let parse_num = |s| {
            let (s, minus) = opt(char('-'))(s)?;
            let (s, num) = map_res(recognize(digit1), str::parse::<i32>)(s)?;
            Ok((s, if minus.is_some() { -num } else { num }))
        };

        let (input, _) = tag("p=")(input)?;
        let (input, x) = parse_num(input)?;
        let (input, _) = tuple((char(','), multispace0))(input)?;
        let (input, y) = parse_num(input)?;
        let (input, _) = tuple((multispace0, tag("v=")))(input)?;
        let (input, vx) = parse_num(input)?;
        let (input, _) = tuple((char(','), multispace0))(input)?;
        let (input, vy) = parse_num(input)?;

        Ok((
            input,
            Robot {
                position: (x, y),
                velocity: (vx, vy),
            },
        ))
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("", "");
        Ok(())
    }
}
