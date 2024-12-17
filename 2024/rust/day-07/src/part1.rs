use nom::{
    bytes::complete::tag,
    character::complete::{space1, u32},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Equation {
    result: u32,
    elements: Vec<u32>,
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, (result, _, elements)) =
        tuple((u32, tag(": "), separated_list1(space1, u32)))(input)?;

    Ok((input, Equation { result, elements }))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let equations: Vec<_> = input
        .lines()
        .filter_map(|line| parse_equation(line).ok())
        .map(|(_, eq)| eq)
        .collect();

    // add (+) and multiply (*)
    for equation in equations {
        dbg!(&equation);
        let result = equation.result;
        let elements = equation.elements;

        let sum = elements.iter().sum::<u32>();
        if sum == result {
            return Ok(format!("{}", result));
        }
    }
    Ok(format!("{:?}", 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("1", process(input)?);
        Ok(())
    }
}
