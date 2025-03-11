use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut state = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    fn traverse(
        state: &mut [usize],
        result: usize,
        visited: &mut HashMap<Vec<usize>, usize>,
    ) -> usize {
        let n = state.len();
        if let Some(prev_result) = visited.get(state) {
            return result - prev_result;
        }

        visited.insert(state.to_vec(), result);

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

    let mut visited = HashMap::new();
    let result = traverse(&mut state, 0, &mut visited);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 2 7 0";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
