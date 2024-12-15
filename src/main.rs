mod days;

use days::*;
use std::fs;

fn main() {
    // Day 1
    let input = fs::read_to_string("inputs/day01.txt").expect("Failed to read input file");
    let result = day01::part1(&input);
    println!("Day 1 Part 1 Result: {}", result);
    let result = day01::part2(&input);
    println!("Day 1 Part 2 Result: {}", result);

    // Day 2
    let input = fs::read_to_string("inputs/day02.txt").expect("Failed to read input file");
    let result = day02::part1(&input);
    println!("Day 2 Part 1 Result: {}", result);
    let result = day02::part2(&input);
    println!("Day 2 Part 2 Result: {}", result);

    // Day 3
    let input = fs::read_to_string("inputs/day03.txt").expect("Failed to read input file");
    let result = day03::part1(&input);
    println!("Day 3 Part 1 Result: {}", result);
    let result = day03::part2(&input);
    println!("Day 3 Part 2 Result: {}", result);

    // Day 4
    let input = fs::read_to_string("inputs/day04.txt").expect("Failed to read input file");
    let result = day04::part1(&input);
    println!("Day 4 Part 1 Result: {}", result);
    let result = day04::part2(&input);
    println!("Day 4 Part 2 Result: {}", result);
}
