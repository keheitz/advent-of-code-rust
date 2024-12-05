use rulinalg::matrix::{BaseMatrix, Matrix};

trait CharMatrixSearch {
    fn find_all_positions(&self, target: char) -> Vec<(usize, usize)>;
}

impl CharMatrixSearch for Matrix<char> {
    fn find_all_positions(&self, target: char) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                if self[[r, c]] == target {
                    positions.push((r, c));
                }
            }
        }
        
        positions
    }
}


#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut xmas_count: i32 = 0;
    
    // Create a flat vector of characters
    let chars: Vec<char> = lines
        .iter()
        .flat_map(|line: &&str| line.chars())
        .collect();
    
    // Create and return the matrix
    let matrix: Matrix<char> = Matrix::new(rows, cols, chars);
    
    // Find all 'M' positions
    let x_positions: Vec<(usize, usize)> = matrix.find_all_positions('X');
    let m_positions: Vec<(usize, usize)> = matrix.find_all_positions('M');
    let a_positions: Vec<(usize, usize)> = matrix.find_all_positions('A');
    let s_positions: Vec<(usize, usize)> = matrix.find_all_positions('S');
    
    println!("Count of 'X': {}", x_positions.len());
    // Is this elegant? No. Is this the only way I personally can think about matrices? Yes.
    for (row, col) in x_positions {
        //Okay, need to see if there is line of adjacent M, A, S vertically, horizontally, diagonally
        // Horizontal forward
        if m_positions.contains(&(row, col + 1)) && a_positions.contains(&(row, col + 2)) && s_positions.contains(&(row, col + 3)) {
            xmas_count += 1;
        } 
        // Horizontal backward
        if m_positions.contains(&(row, col - 1)) && a_positions.contains(&(row, col - 2)) && s_positions.contains(&(row, col - 3)) {
            xmas_count += 1;
        }
        // Vertical down
        if m_positions.contains(&(row + 1, col)) && a_positions.contains(&(row + 2, col)) && s_positions.contains(&(row + 3, col)) {
            xmas_count += 1;
        } 
        // Vertical up
        if m_positions.contains(&(row - 1, col)) && a_positions.contains(&(row - 2, col)) && s_positions.contains(&(row - 3, col)) {
            xmas_count += 1;
        }
        // Diagonal up right
        if m_positions.contains(&(row + 1, col + 1)) && a_positions.contains(&(row + 2, col + 2)) && s_positions.contains(&(row + 3, col + 3)) {
            xmas_count += 1;
        } 
        // Diagonal up left
        if m_positions.contains(&(row - 1, col + 1)) && a_positions.contains(&(row - 2, col + 2)) && s_positions.contains(&(row - 3, col + 3)) {
            xmas_count += 1;
        }
        // Diagonal down right
        if m_positions.contains(&(row + 1, col - 1)) && a_positions.contains(&(row + 2, col - 2)) && s_positions.contains(&(row + 3, col - 3)) {
            xmas_count += 1;
        }
        // Diagonal down left
        if m_positions.contains(&(row - 1, col - 1)) && a_positions.contains(&(row - 2, col - 2)) && s_positions.contains(&(row - 3, col - 3)) {
            xmas_count += 1;
        }
    }

    return xmas_count;
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut mas_count: i32 = 0;
    
    // Create a flat vector of characters
    let chars: Vec<char> = lines
        .iter()
        .flat_map(|line| line.chars())
        .collect();
    
    // Create and return the matrix
    let matrix: Matrix<char> = Matrix::new(rows, cols, chars);
    
    // Find all 'M' positions
    let m_positions: Vec<(usize, usize)> = matrix.find_all_positions('M');
    let a_positions: Vec<(usize, usize)> = matrix.find_all_positions('A');
    let s_positions: Vec<(usize, usize)> = matrix.find_all_positions('S');
    
    for (row, col) in a_positions {
        if m_positions.contains(&(row - 1, col + 1)) && s_positions.contains(&(row + 1, col - 1)) {
            if m_positions.contains(&(row + 1, col + 1)) && s_positions.contains(&(row - 1, col - 1)) {
                mas_count += 1;
            } 
            if m_positions.contains(&(row - 1, col - 1)) && s_positions.contains(&(row + 1, col + 1)) {
                mas_count += 1;
            }
        }
        if m_positions.contains(&(row + 1, col - 1)) && s_positions.contains(&(row - 1, col + 1)) {
            if m_positions.contains(&(row - 1, col - 1)) && s_positions.contains(&(row + 1, col + 1)) {
                mas_count += 1;
            } 
            if m_positions.contains(&(row + 1, col + 1)) && s_positions.contains(&(row - 1, col - 1)) {
                mas_count += 1;
            }
        }
    }

    return mas_count;
}
