#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut robots = Robot::parse(input);

    const ROW: i32 = 7;
    const COL: i32 = 11;

    // use this for cargo run
    // const ROW: i32 = 103;
    // const COL: i32 = 101;

    const TIME_STEP: i32 = 100;
    const MID_ROW: i32 = ROW / 2;
    const MID_COL: i32 = COL / 2;

    for robot in robots.iter_mut() {
        let (x, y) = robot.position;
        let (vx, vy) = robot.velocity;

        robot.position = (
            (x + vx * TIME_STEP).rem_euclid(COL),
            (y + vy * TIME_STEP).rem_euclid(ROW),
        );
    }

    // let mut grid = vec![vec![0; COL as usize]; ROW as usize];

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for robot in &robots {
        let (x, y) = robot.position;

        if (0..MID_ROW).contains(&y) && (0..MID_COL).contains(&x) {
            q1 += 1;
        } else if (0..MID_ROW).contains(&y) && (MID_COL + 1..COL).contains(&x) {
            q2 += 1;
        } else if (MID_ROW + 1..ROW).contains(&y) && (0..MID_COL).contains(&x) {
            q3 += 1;
        } else if (MID_ROW + 1..ROW).contains(&y) && (MID_COL + 1..COL).contains(&x) {
            q4 += 1;
        }
    }

    Ok((q1 * q2 * q3 * q4).to_string())
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
        assert_eq!("12", process(input)?);
        Ok(())
    }
}
