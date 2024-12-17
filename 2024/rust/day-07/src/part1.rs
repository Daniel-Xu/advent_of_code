use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Equation {
    result: u64,
    elements: Vec<u64>,
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, (result, _, elements)) =
        tuple((digit1, tag(": "), separated_list1(space1, digit1)))(input)?;

    Ok((
        input,
        Equation {
            result: result.parse().unwrap(),
            elements: elements.iter().map(|n| n.parse().unwrap()).collect(),
        },
    ))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let equations: Vec<_> = input
        .lines()
        .filter_map(|line| parse_equation(line).ok())
        .map(|(_, eq)| eq)
        .collect();

    // add (+) and multiply (*)
    let mut fin_res: u64 = 0;

    for equation in equations {
        let result = equation.result;
        let elements = equation.elements;
        let mut path = vec![];

        if traverse(&elements, &mut path, elements[0], 1, result).is_some() {
            fin_res += result;
        }
    }
    Ok(format!("{:?}", fin_res))
}

fn traverse(
    elements: &Vec<u64>,
    path: &mut Vec<char>,
    mut result: u64,
    depth: usize,
    expected: u64,
) -> Option<u64> {
    if result > expected {
        return None;
    }

    if path.len() == elements.len() - 1 {
        return Some(result);
    }

    for operator in ['+', '*'] {
        path.push(operator);
        match operator {
            '+' => result += elements[depth],
            '*' => result *= elements[depth],
            _ => unreachable!(),
        };

        if let Some(v) = traverse(elements, path, result, depth + 1, expected) {
            if v == expected {
                return Some(v);
            }
        }

        path.pop();

        match operator {
            '+' => result -= elements[depth],
            '*' => result /= elements[depth],
            _ => (),
        };
    }

    None
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
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
