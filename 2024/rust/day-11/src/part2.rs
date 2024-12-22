use rayon::prelude::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};

fn process_single_number(initial: u64, iterations: usize) -> usize {
    let mut nums = Vec::new();
    nums.push(initial);
    let count = AtomicUsize::new(1);

    for i in 0..iterations {
        let current_nums = nums.clone();
        nums.clear();

        let new_numbers: Vec<u64> = current_nums
            .par_iter()
            .flat_map(|&current| {
                let mut local_nums = Vec::new();

                if current == 0 {
                    local_nums.push(1);
                } else {
                    let s = current.to_string();
                    if s.len() % 2 == 0 {
                        let (a, b) = s.split_at(s.len() / 2);
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<u64>(), b.parse::<u64>()) {
                            local_nums.push(a_num);
                            local_nums.push(b_num);
                            count.fetch_add(1, Ordering::Relaxed);
                        }
                    } else {
                        local_nums.push(current * 2024);
                    }
                }
                local_nums
            })
            .collect();

        nums.extend(new_numbers);

        if i % 10 == 0 {
            println!(
                "Number: {}, Iteration: {}, Count: {}, Current size: {}",
                initial,
                i,
                count.load(Ordering::Relaxed),
                nums.len()
            );
        }
    }

    count.load(Ordering::Relaxed)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let nums = input
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<VecDeque<u64>>();

    // iter 0
    // iter 1
    // iter 2
    let final_counts: Vec<usize> = nums
        .par_iter() // Pallel iterator
        .map(|&num| process_single_number(num, 75))
        .collect();

    // Sum up all the results
    let total = final_counts.iter().sum::<usize>();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
