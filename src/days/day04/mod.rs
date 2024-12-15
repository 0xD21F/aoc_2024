pub fn part1(input: &str) -> String {
    // First, convert the input into a 2D grid of characters for easier processing
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // These vectors represent all eight possible directions:
    // right, down-right, down, down-left, left, up-left, up, up-right
    let directions = [
        (0, 1),   // right
        (1, 1),   // down-right
        (1, 0),   // down
        (1, -1),  // down-left
        (0, -1),  // left
        (-1, -1), // up-left
        (-1, 0),  // up
        (-1, 1),  // up-right
    ];

    // For each position in the grid
    for row in 0..rows {
        for col in 0..cols {
            // Try each direction from this position
            for &(dr, dc) in &directions {
                if check_xmas(&grid, row, col, dr, dc) {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

pub fn part2(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check every possible center 'A'
    // We need room for a full 3x3 pattern, so we skip edges
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            // Only process if we've found an 'A'
            if grid[row][col] == 'A' {
                if is_x_mas_pattern(&grid, row, col) {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

fn check_xmas(grid: &[Vec<char>], row: usize, col: usize, dr: i32, dc: i32) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let word: Vec<char> = "XMAS".chars().collect();

    // Check 4 positions in the given direction (XMAS is 4 letters)
    for i in 0_usize..4 {
        // Calculate the new position by:
        // - Converting i to i32 for multiplication with our direction
        // - Multiplying by our direction values (dr, dc)
        // - Adding to our starting position
        let new_row = row as i32 + dr * (i as i32);
        let new_col = col as i32 + dc * (i as i32);

        // Check if we're still within the grid boundaries
        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false; // We've gone outside the grid, so this can't be a valid XMAS
        }

        // Convert our calculated positions to usize for array indexing
        // We can safely unwrap here because we just checked the bounds
        let row_idx: usize = new_row.try_into().unwrap();
        let col_idx: usize = new_col.try_into().unwrap();

        // Compare the character in our grid to the corresponding letter in XMAS
        // If they don't match, this isn't a valid XMAS sequence
        if grid[row_idx][col_idx] != word[i] {
            return false;
        }
    }

    // If we made it through all 4 positions without returning false,
    // we've found a valid XMAS sequence
    true
}

fn is_x_mas_pattern(grid: &[Vec<char>], center_row: usize, center_col: usize) -> bool {
    // Check the exact pattern positions for a 3x3 X shape
    let top_left = grid[center_row - 1][center_col - 1];
    let top_right = grid[center_row - 1][center_col + 1];
    let bottom_left = grid[center_row + 1][center_col - 1];
    let bottom_right = grid[center_row + 1][center_col + 1];

    // We need both diagonals to be MAS or SAM
    let diagonal1 = vec![top_left, grid[center_row][center_col], bottom_right];
    let diagonal2 = vec![top_right, grid[center_row][center_col], bottom_left];

    // Both diagonals must form valid MAS patterns
    is_mas(&diagonal1) && is_mas(&diagonal2)
}

fn is_mas(chars: &[char]) -> bool {
    chars == &['M', 'A', 'S'] || chars == &['S', 'A', 'M']
}
