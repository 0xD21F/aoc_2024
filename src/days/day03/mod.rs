use regex::Regex;

pub fn part1(input: &str) -> String {
    // Create a regular expression to match valid mul instructions
    // Explanation of regex pattern:
    // mul     - Matches exactly "mul"
    // \(      - Matches opening parenthesis
    // (\d{1,3}) - Captures 1-3 digits (first number)
    // ,       - Matches comma
    // (\d{1,3}) - Captures 1-3 digits (second number)
    // \)      - Matches closing parenthesis
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Find all valid multiplications and sum their results
    let sum: i32 = pattern
        .captures_iter(input)
        .map(|cap| {
            // Parse the two numbers from the captured groups
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y // Multiply them
        })
        .sum();

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    // Define patterns for all instruction types we need to match
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();

    // Use this to track each instruction's position and type
    #[derive(Debug)]
    enum Instruction {
        Multiply(usize, i32, i32), // position, x, y
        Do(usize),                 // position
        Dont(usize),               // position
    }

    // Collect all instructions with their positions
    let mut instructions = Vec::new();

    // Find all multiplication instructions
    for cap in mul_pattern.captures_iter(input) {
        let pos = cap.get(0).unwrap().start();
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        instructions.push(Instruction::Multiply(pos, x, y));
    }

    // Find all do() instructions
    for mat in do_pattern.find_iter(input) {
        instructions.push(Instruction::Do(mat.start()));
    }

    // Find all don't() instructions
    for mat in dont_pattern.find_iter(input) {
        instructions.push(Instruction::Dont(mat.start()));
    }

    // Sort instructions by position to process them in order
    instructions.sort_by_key(|inst| match inst {
        Instruction::Multiply(pos, _, _) => *pos,
        Instruction::Do(pos) => *pos,
        Instruction::Dont(pos) => *pos,
    });

    // Process instructions in order, tracking enabled state
    let mut enabled = true;
    let mut sum = 0;

    for inst in instructions {
        match inst {
            Instruction::Multiply(_, x, y) => {
                if enabled {
                    sum += x * y;
                }
            }
            Instruction::Do(_) => enabled = true,
            Instruction::Dont(_) => enabled = false,
        }
    }

    sum.to_string()
}
