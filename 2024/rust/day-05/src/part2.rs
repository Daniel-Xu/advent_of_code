use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Rule {
    from: u32,
    to: u32,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (from, to)) = separated_pair(digit1, char('|'), digit1)(input)?;
    Ok((
        input,
        Rule {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        },
    ))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, nums) = separated_list1(char(','), digit1)(input)?;
    Ok((
        input,
        nums.into_iter()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>(),
    ))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Vec<u32>>)> {
    let (input, rules) = separated_list1(newline, parse_rule)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, numbers) = separated_list1(newline, parse_numbers)(input)?;
    Ok((input, (rules, numbers)))
}

pub fn process(input: &str) -> miette::Result<String> {
    let (_, (rules, numbers)) = parse_input(input).unwrap();
    let mut rule_map = HashMap::new();
    for rule in rules {
        // a -> elements need to be after a
        rule_map
            .entry(rule.from)
            .or_insert(HashSet::new())
            .insert(rule.to);
    }

    let mut incorrect_orders = vec![];

    for seq in numbers {
        'seq: for i in 0..seq.len() {
            for j in i + 1..seq.len() {
                if let Some(dependant) = rule_map.get(&seq[j]) {
                    if dependant.contains(&seq[i]) {
                        incorrect_orders.push(seq);
                        break 'seq;
                    }
                }
            }
        }
    }

    process_incorrect_orders(incorrect_orders, rule_map)
}

fn process_incorrect_orders(
    mut orders: Vec<Vec<u32>>,
    rule_map: HashMap<u32, HashSet<u32>>,
) -> miette::Result<String> {
    let mut res = 0;
    for order in orders.iter_mut() {
        let mut i = 0;
        while i < order.len() {
            let mut switch = false;
            for j in i + 1..order.len() {
                if let Some(dependant) = rule_map.get(&order[j]) {
                    if dependant.contains(&order[i]) {
                        order.swap(i, j);
                        switch = true;
                    }
                }
            }
            if !switch {
                i += 1;
            }
        }
        res += order[order.len() / 2];
    }
    Ok(res.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
