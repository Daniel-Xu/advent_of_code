use miette::{miette, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, space1},
    combinator::{map_res, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::HashMap;

fn parse_name(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_weight(input: &str) -> IResult<&str, u32> {
    map_res(delimited(tag("("), digit1, tag(")")), |s: &str| {
        s.parse::<u32>()
    })(input)
}

fn parse_children(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = tuple((space1, tag("->"), space1))(input)?;
    separated_list0(tuple((tag(","), multispace0)), parse_name)(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    let (input, name) = parse_name(input)?;
    let (input, _) = space1(input)?;
    let (input, weight) = parse_weight(input)?;
    let (input, children) = opt(parse_children)(input)?;

    Ok((input, (name, weight, children.unwrap_or_default())))
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut in_degree: HashMap<&str, u32> = HashMap::new();

    for line in input.lines() {
        if let Ok((_, (name, _, children))) = parse_line(line) {
            for child in children {
                *in_degree.entry(child).or_insert(0) += 1;
                in_degree.entry(name).or_insert(0);
            }
        }
    }

    in_degree
        .iter()
        .find_map(|(&name, &degree)| {
            if degree == 0 {
                Some(name.to_owned())
            } else {
                None
            }
        })
        .ok_or(miette!("cant find root"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!("tknk", process(input)?);
        Ok(())
    }
}
