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
    use crate::part1;
    let start_node = part1::process(input)?;

    let mut neib = HashMap::new();
    let mut wght = HashMap::new();

    for line in input.lines() {
        if let Ok((_, (name, weight, children))) = parse_line(line) {
            wght.entry(name).or_insert(weight);
            for child in children {
                neib.entry(name).or_insert(Vec::new()).push(child);
            }
        }
    }
    // dbg!(&wght);

    fn dfs(start: &str, w: &HashMap<&str, u32>, neib: &HashMap<&str, Vec<&str>>) -> Result<u32> {
        match neib.get(start) {
            None => Ok(*w.get(start).unwrap()),
            Some(children) => {
                let mut history = vec![];
                for &name in children {
                    let res = dfs(name, w, neib)?;
                    history.push(res);
                }

                let first_v = *history.first().unwrap();
                let all_same = history.iter().all(|&x| x == first_v);

                if all_same {
                    Ok(w.get(start).unwrap() + history.iter().sum::<u32>())
                } else {
                    let different_v = *history.iter().find(|&&x| x != first_v).unwrap();
                    let diff = different_v.abs_diff(first_v);
                    let (_normal, outlier) =
                        if history.iter().filter(|&&x| x == first_v).count() > 1 {
                            (first_v, different_v)
                        } else {
                            (different_v, first_v)
                        };

                    let result = history
                        .iter()
                        .position(|&x| x == outlier)
                        .and_then(|pos| children.get(pos))
                        .and_then(|name| w.get(name))
                        .map(|&x| x - diff)
                        .unwrap();

                    Err(miette!("{}", result))
                }
            }
        }
    }

    let result = match dfs(&start_node, &wght, &neib) {
        Ok(_) => "0".to_string(),
        Err(e) => e.to_string(),
    };

    Ok(result)
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
        assert_eq!("60", process(input)?);
        Ok(())
    }
}
