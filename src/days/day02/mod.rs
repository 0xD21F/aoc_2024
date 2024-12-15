pub fn part1(input: &str) -> String {
    // Count how many sequences in the input are safe
    let safe_count = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|&line| is_sequence_safe(parse_line(line)))
        .count();

    safe_count.to_string()
}

pub fn part2(input: &str) -> String {
    // Note: Current implementation checks every possible number removal.
    // A more efficient solution could count violations and their positions
    // to determine if a sequence is fixable, reducing complexity from O(n²) to O(n).
    let safe_count = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|&line| is_sequence_safe_with_dampener(parse_line(line)))
        .count();

    safe_count.to_string()
}

fn parse_line(line: &str) -> Vec<i32> {
    // Convert a space-separated string of numbers into a vector of integers
    line.split_whitespace()
        .map(|n| n.parse().expect("Failed to parse number"))
        .collect()
}

fn is_sequence_safe(levels: Vec<i32>) -> bool {
    // A sequence of length 1 or 0 is technically safe as it can't violate any rules
    if levels.len() <= 1 {
        return true;
    }

    // Check if the sequence is increasing or decreasing by looking at first two numbers
    let is_increasing = levels[1] > levels[0];

    // Use windows(2) to look at adjacent pairs of numbers
    levels.windows(2).all(|pair| {
        let difference = pair[1] - pair[0];

        // For a sequence to be safe:
        // 1. If it started increasing, all pairs must increase
        // 2. If it started decreasing, all pairs must decrease
        // 3. The difference between adjacent numbers must be between 1 and 3 inclusive
        if is_increasing {
            difference >= 1 && difference <= 3
        } else {
            difference <= -1 && difference >= -3
        }
    })
}

fn is_sequence_safe_with_dampener(levels: Vec<i32>) -> bool {
    // First, check if the sequence is already safe without removing anything
    if is_sequence_safe(levels.clone()) {
        return true;
    }

    // If not safe, try removing each number one at a time
    for skip_index in 0..levels.len() {
        // Create a new sequence without the number at skip_index
        let modified_sequence: Vec<i32> = levels
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != skip_index)
            .map(|(_, &num)| num)
            .collect();

        // Check if this modified sequence is safe
        if is_sequence_safe(modified_sequence) {
            return true;
        }
    }

    // If we haven't found a safe sequence after trying all possibilities,
    // then this sequence is unsafe even with the Problem Dampener
    false
}
