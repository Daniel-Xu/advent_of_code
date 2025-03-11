use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    value: i32,
}

#[derive(Debug)]
struct Spiral {
    current: Position,
    direction: Direction,
    turn_taken: i32,
    steps_in_direction: i32,
    steps_before_turn: i32,
    visited: HashMap<(i32, i32), i32>,
}

impl Spiral {
    fn new() -> Self {
        let mut visited = HashMap::new();
        visited.insert((0, 0), 1);
        Spiral {
            current: Position {
                x: 0,
                y: 0,
                value: 1,
            },
            direction: Direction::Right,
            // every 2 turn, we add 1 to steps_before_turn (the side length)
            turn_taken: 0,
            steps_in_direction: 0,
            steps_before_turn: 1,
            visited,
        }
    }

    fn next(&mut self) -> Position {
        // Store current position
        // Move to next position
        match self.direction {
            Direction::Right => self.current.x += 1,
            Direction::Up => self.current.y += 1,
            Direction::Left => self.current.x -= 1,
            Direction::Down => self.current.y -= 1,
        }

        let mut v = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                if let Some(value) = self.visited.get(&(self.current.x + i, self.current.y + j)) {
                    v += value;
                }
            }
        }
        self.current.value = v;
        self.visited.insert((self.current.x, self.current.y), v);

        // Check if we need to turn
        self.steps_in_direction += 1;
        if self.steps_in_direction >= self.steps_before_turn {
            self.turn();
            self.steps_in_direction = 0;

            // Increase steps_before_turn every two turns
            if self.turn_taken == 2 {
                self.turn_taken = 0;
                self.steps_before_turn += 1;
            }
        }

        self.current
    }

    fn turn(&mut self) {
        self.turn_taken += 1;
        self.direction = match self.direction {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        };
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // map to store all available points
    let num = input.trim().parse::<i32>().unwrap();
    let mut spiral = Spiral::new();

    // Print first few positions to demonstrate the pattern
    let res;

    loop {
        let pos = spiral.next();
        if pos.value > num {
            res = pos.value;
            break;
        }
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1024";
        assert_eq!("1968", process(input)?);
        Ok(())
    }
}
