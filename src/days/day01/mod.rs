use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    // Parse the input into two vectors of numbers
    let (left, right): (Vec<i32>, Vec<i32>) = parse_input(input);

    // Calculate the total distance between sorted lists
    let total_distance = calculate_distance(left, right);

    total_distance.to_string()
}

pub fn part2(input: &str) -> String {
    // Parse input the same way as part 1
    let (left, right): (Vec<i32>, Vec<i32>) = parse_input(input);

    // Calculate similarity score
    let similarity_score = calculate_similarity(left, right);

    similarity_score.to_string()
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    // Split input into lines and parse each line into two numbers
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Split line into two numbers and parse them
        let mut numbers = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().expect("Failed to parse number"));

        // Extract the two numbers
        left.push(numbers.next().expect("Missing left number"));
        right.push(numbers.next().expect("Missing right number"));
    }

    (left, right)
}

fn calculate_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    // Sort both lists independently
    left.sort();
    right.sort();

    // Zip the sorted lists together and sum the absolute differences
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn calculate_similarity(left: Vec<i32>, right: Vec<i32>) -> i32 {
    // Create a frequency map of numbers in the right list
    let right_frequencies: HashMap<i32, i32> =
        right.iter().fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

    // For each number in the left list:
    // - Look up how many times it appears in right list (0 if not present)
    // - Multiply the number by its frequency
    // - Sum all these products
    left.iter()
        .map(|&num| {
            let frequency = right_frequencies.get(&num).copied().unwrap_or(0);
            num * frequency
        })
        .sum()
}
