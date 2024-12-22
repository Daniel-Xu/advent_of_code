use num::integer::lcm;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let button_presses = ButtonPress::parse(input);

    let mut result = 0i128;
    for button in button_presses {
        let (x1, y1) = button.button_a;
        let (x2, y2) = button.button_b;
        let (targetx, targety) = button.prize;

        let lcm_a = lcm(x1, y1);

        let num1 = targetx * lcm_a / x1 - targety * lcm_a / y1;
        let num2 = x2 * lcm_a / x1 - y2 * lcm_a / y1;

        if num1 % num2 != 0 {
            continue;
        } else {
            let num_b = num1 / num2;
            let rest = targetx - x2 * num_b;
            if rest % x1 == 0 {
                let num_a = rest / x1;
                result += num_a * 3;
                result += num_b;
            } else {
                continue;
            }
        }
    }

    Ok(result.to_string())
}

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map_res, opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct ButtonPress {
    button_a: (i128, i128),
    button_b: (i128, i128),
    prize: (i128, i128),
}

impl ButtonPress {
    fn parse(input: &str) -> Vec<ButtonPress> {
        input
            .split("\n\n")
            .map(|block| {
                let (_, press) = Self::parse_single(block).unwrap();
                press
            })
            .collect::<Vec<_>>()
    }

    fn parse_single(input: &str) -> IResult<&str, ButtonPress> {
        let (input, button_a) = Self::parse_button_a(input)?;
        let (input, button_b) = Self::parse_button_b(input)?;
        let (input, prize) = Self::parse_prize(input)?;

        Ok((
            input,
            ButtonPress {
                button_a,
                button_b,
                prize,
            },
        ))
    }

    fn parse_coords(input: &str) -> IResult<&str, (i128, i128)> {
        let parse_num = |s| {
            let (s, _) = opt(char('+'))(s)?;
            let (s, num) = map_res(recognize(digit1), str::parse::<i128>)(s)?;
            Ok((s, num))
        };

        let (input, x) = preceded(tag("X"), parse_num)(input)?;
        let (input, _) = tuple((char(','), multispace0))(input)?;
        let (input, y) = preceded(tag("Y"), parse_num)(input)?;

        Ok((input, (x, y)))
    }

    fn parse_button_a(input: &str) -> IResult<&str, (i128, i128)> {
        preceded(tag("Button A:"), preceded(multispace1, Self::parse_coords))(input)
    }

    fn parse_button_b(input: &str) -> IResult<&str, (i128, i128)> {
        preceded(
            tuple((multispace0, tag("Button B:"))),
            preceded(multispace1, Self::parse_coords),
        )(input)
    }

    fn parse_prize(input: &str) -> IResult<&str, (i128, i128)> {
        preceded(
            tuple((char('\n'), tag("Prize:"))),
            preceded(multispace1, Self::parse_equal),
        )(input)
    }

    fn parse_equal(input: &str) -> IResult<&str, (i128, i128)> {
        let parse_num = |s| {
            let (s, num) = map_res(recognize(digit1), str::parse::<i128>)(s)?;
            Ok((s, num + 10000000000000))
        };

        let (input, x) = preceded(tag("X="), parse_num)(input)?;
        let (input, _) = tuple((char(','), multispace0))(input)?;
        let (input, y) = preceded(tag("Y="), parse_num)(input)?;

        Ok((input, (x, y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("875318608908", process(input)?);
        Ok(())
    }
}
