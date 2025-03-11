use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut state = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    fn traverse(state: &mut [usize], result: usize, visited: &mut HashSet<Vec<usize>>) -> usize {
        let n = state.len();
        if visited.contains(state) {
            return result;
        }

        visited.insert(state.to_vec());

        let max_index = state
            .iter()
            .position(|&x| x == *state.iter().max().unwrap())
            .unwrap();
        let max_value = state[max_index];
        state[max_index] = 0;

        for i in 0..max_value {
            state[(max_index + i + 1) % n] += 1;
        }

        traverse(state, result + 1, visited)
    }

    let mut visited = HashSet::new();
    let result = traverse(&mut state, 0, &mut visited);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 2 7 0";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
