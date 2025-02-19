#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut robots = Robot::parse(input);

    // const ROW: i32 = 103;
    // const COL: i32 = 101;
    const ROW: i32 = 7;
    const COL: i32 = 11;

    const TIME_STEP: i32 = 100;
    const MID_ROW: i32 = ROW / 2;
    const MID_COL: i32 = COL / 2;

    for t in 0..501 {
        for robot in robots.iter_mut() {
            let (x, y) = robot.position;
            let (vx, vy) = robot.velocity;

            robot.position = ((x + vx * t).rem_euclid(COL), (y + vy * t).rem_euclid(ROW));
        }

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
        println!("----------> ");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    Ok(0.to_string())
}

#[derive(Debug)]
struct Robot {
    // x is col, y is row
    position: (i32, i32),
    velocity: (i32, i32),
}

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
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
