#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut state = input
        .trim()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    fn traverse(state: &mut [i32], move_step: i32, result: u32) -> u32 {
        // dbg!(&state, move_step, result);
        // println!("state {:?}, move_step {}, result {}", state, move_step, result);
        if move_step >= state.len() as i32 || move_step < 0 {
            return result;
        }

        let original_move_step = state[move_step as usize];
        if original_move_step >= 3 {
            state[move_step as usize] -= 1;
        } else {
            state[move_step as usize] += 1;
        }
        traverse(state, move_step + original_move_step, result + 1)
    }

    Ok(traverse(&mut state, 0, 0).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0
3
0
1
-3";
        assert_eq!("10", process(input)?);
        Ok(())
    }
}
